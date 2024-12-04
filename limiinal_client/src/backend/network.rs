use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{Hash, Hasher},
    str::FromStr,
    time::Duration,
};

use futures_timer::Delay;

use clap::Parser;
use futures::{executor::block_on, future::FutureExt, stream::StreamExt};
use libp2p::{
    core::multiaddr::{Multiaddr, Protocol},
    dcutr, gossipsub, identify, identity, noise, ping, relay,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId,
};
use tokio::{io, io::AsyncBufReadExt, select, task};
use tracing_subscriber::{filter, EnvFilter};

#[derive(Debug, Parser)]
#[clap(name = "libp2p DCUtR client")]
struct Opts {
    /// The mode (client-listen, client-dial).
    #[clap(long)]
    mode: Mode,

    /// Fixed value to generate deterministic peer id.
    #[clap(long)]
    secret_key_seed: u8,

    /// The listening address
    #[clap(long)]
    relay_address: Multiaddr,

    /// Peer ID of the remote peer to hole punch to.
    #[clap(long)]
    remote_peer_id: Option<PeerId>,
}

#[derive(Clone, Debug, PartialEq, Parser)]
enum Mode {
    Dial,
    Listen,
}

impl FromStr for Mode {
    type Err = String;
    fn from_str(mode: &str) -> Result<Self, Self::Err> {
        match mode {
            "dial" => Ok(Mode::Dial),
            "listen" => Ok(Mode::Listen),
            _ => Err("Expected either 'dial' or 'listen'".to_string()),
        }
    }
}

pub struct AppCore {
    pub backend_thread: Option<task::JoinHandle<()>>,
}

impl AppCore {
    pub fn new() -> Self {
        AppCore {
            backend_thread: None,
        }
    }

    pub async fn run(&mut self) {
        if let Err(e) = AppCore::start().await {
            tracing::error!("Failed to start AppCore: {:?}", e);
        }
    }

    async fn start() -> Result<(), Box<dyn Error>> {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init();

        let opts = Opts::parse();

        #[derive(NetworkBehaviour)]
        struct Behaviour {
            relay_client: relay::client::Behaviour,
            ping: ping::Behaviour,
            identify: identify::Behaviour,
            dcutr: dcutr::Behaviour,
            gossipsub: gossipsub::Behaviour,
        }

        let mut swarm =
            libp2p::SwarmBuilder::with_existing_identity(generate_ed25519(opts.secret_key_seed))
                .with_tokio()
                .with_tcp(
                    tcp::Config::default().nodelay(true),
                    noise::Config::new,
                    yamux::Config::default,
                )?
                .with_quic()
                .with_dns()?
                .with_relay_client(noise::Config::new, yamux::Config::default)?
                .with_behaviour(|keypair, relay_behaviour| Behaviour {
                    relay_client: relay_behaviour,
                    ping: ping::Behaviour::new(ping::Config::new()),
                    identify: identify::Behaviour::new(identify::Config::new(
                        "/TODO/0.0.1".to_string(),
                        keypair.public(),
                    )),
                    dcutr: dcutr::Behaviour::new(keypair.public().to_peer_id()),
                    gossipsub: {
                        let gossipsub_config = gossipsub::ConfigBuilder::default()
                            .heartbeat_interval(Duration::from_secs(1))
                            .validation_mode(gossipsub::ValidationMode::Strict)
                            .build()
                            .expect("Valid Gossipsub configuration");
                        gossipsub::Behaviour::new(
                            gossipsub::MessageAuthenticity::Signed(keypair.clone()),
                            gossipsub_config,
                        )
                        .expect("Failed to create Gossipsub behaviour")
                    },
                })?
                .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
                .build();

        swarm
            .listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse().unwrap())
            .unwrap();
        swarm
            .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
            .unwrap();

        let topic = gossipsub::IdentTopic::new("example-topic");
        if swarm.behaviour_mut().gossipsub.subscribe(&topic).is_err() {
            tracing::error!("Failed to subscribe to topic");
        }

        // Wait to listen on all interfaces.
        block_on(async {
            let mut delay = futures_timer::Delay::new(std::time::Duration::from_secs(1)).fuse();
            loop {
                futures::select! {
                    event = swarm.next() => {
                        match event.unwrap() {
                            SwarmEvent::NewListenAddr { address, .. } => {
                                tracing::info!(%address, "Listening on address");
                            }
                            event => panic!("{event:?}"),
                        }
                    }
                    _ = delay => {
                        // Likely listening on all interfaces now, thus continuing by breaking the loop.
                        break;
                    }
                }
            }
        });

        // Connect to the relay server. Not for the reservation or relayed connection, but to (a) learn
        // our local public address and (b) enable a freshly started relay to learn its public address.
        swarm.dial(opts.relay_address.clone()).unwrap();
        block_on(async {
            let mut learned_observed_addr = false;
            let mut told_relay_observed_addr = false;

            loop {
                match swarm.next().await.unwrap() {
                    SwarmEvent::NewListenAddr { .. } => {}
                    SwarmEvent::Dialing { .. } => {}
                    SwarmEvent::ConnectionEstablished { .. } => {}
                    SwarmEvent::Behaviour(BehaviourEvent::Ping(_)) => {}
                    SwarmEvent::Behaviour(BehaviourEvent::Identify(identify::Event::Sent {
                        ..
                    })) => {
                        tracing::info!("Told relay its public address");
                        told_relay_observed_addr = true;
                    }
                    SwarmEvent::Behaviour(BehaviourEvent::Identify(
                        identify::Event::Received {
                            info: identify::Info { observed_addr, .. },
                            ..
                        },
                    )) => {
                        tracing::info!(address=%observed_addr, "Relay told us our observed address");
                        learned_observed_addr = true;
                    }
                    event => {
                        tracing::error!(?event, "Unhandled event occurred");
                    }
                }

                if learned_observed_addr && told_relay_observed_addr {
                    break;
                }
            }
        });

        match opts.mode {
            Mode::Dial => {
                swarm
                    .dial(
                        opts.relay_address
                            .with(Protocol::P2pCircuit)
                            .with(Protocol::P2p(opts.remote_peer_id.unwrap())),
                    )
                    .unwrap();
            }
            Mode::Listen => {
                swarm
                    .listen_on(opts.relay_address.with(Protocol::P2pCircuit))
                    .unwrap();
            }
        }

        block_on(async {
            // Create an asynchronous stdin reader
            let stdin = io::BufReader::new(io::stdin());
            let mut lines = stdin.lines();

            // Define the Gossipsub topic
            let topic = gossipsub::IdentTopic::new("example-topic");

            loop {
                tokio::select! {
                    // Handle Gossipsub and swarm events
                    event = swarm.next() => match event.unwrap() {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            tracing::info!(%address, "Listening on address");
                        }
                        SwarmEvent::Behaviour(BehaviourEvent::RelayClient(
                            relay::client::Event::ReservationReqAccepted { .. },
                        )) => {
                            assert!(opts.mode == Mode::Listen);
                            tracing::info!("Relay accepted our reservation request");
                        }
                        SwarmEvent::Behaviour(BehaviourEvent::RelayClient(event)) => {
                            tracing::info!(?event)
                        }
                        SwarmEvent::Behaviour(BehaviourEvent::Dcutr(event)) => {
                            tracing::info!(?event)
                        }
                        SwarmEvent::Behaviour(BehaviourEvent::Identify(event)) => {
                            tracing::info!(?event)
                        }
                        SwarmEvent::Behaviour(BehaviourEvent::Ping(_)) => {}
                        SwarmEvent::ConnectionEstablished {
                            peer_id, endpoint, ..
                        } => {
                            tracing::info!(peer=%peer_id, ?endpoint, "Established new connection");
                        }
                        SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                            tracing::info!(peer=?peer_id, "Outgoing connection failed: {error}");
                        }
                        SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(
                            gossipsub::Event::Message {
                                propagation_source,
                                message_id,
                                message,
                            },
                        )) => {
                            tracing::info!(
                                "Received: '{:?}' from {:?}",
                                String::from_utf8_lossy(&message.data),
                                propagation_source
                            );
                        }
                        SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(
                            gossipsub::Event::Subscribed { peer_id, topic },
                        )) => {
                            println!("{:?} subscribed to {:?}", peer_id, topic);
                        }
                        SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(
                            gossipsub::Event::Unsubscribed { peer_id, topic },
                        )) => {
                            println!("{:?} unsubscribed from {:?}", peer_id, topic);
                        }
                        _ => {}
                    },
                    // Read lines from stdin
                    line = lines.next_line() => match line {
                        Ok(Some(text)) => {
                            // Publish the input text to the Gossipsub topic
                            if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), text.clone().into_bytes()) {
                                println!("Failed to publish message: {:?}", e);
                            } else {
                                println!("Published message: {:?}", text);
                            }
                        }
                        Ok(None) => {
                            tracing::warn!("Stdin closed");
                            break; // Exit the loop if stdin is closed
                        }
                        Err(e) => {
                            tracing::error!("Failed to read from stdin: {:?}", e);
                        }
                    }
                }
            }
        });
        Ok(())
    }
}

fn generate_ed25519(secret_key_seed: u8) -> identity::Keypair {
    let mut bytes = [0u8; 32];
    bytes[0] = secret_key_seed;

    identity::Keypair::ed25519_from_bytes(bytes).expect("only errors on wrong length")
}
