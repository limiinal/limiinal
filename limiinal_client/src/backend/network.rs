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

#[derive(NetworkBehaviour)]
struct Behaviour {
    gossipsub: gossipsub::Behaviour,
    relay_client: relay::client::Behaviour,
    ping: ping::Behaviour,
    identify: identify::Behaviour,
    dcutr: dcutr::Behaviour,
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
        self.backend_thread = Some(task::spawn(async {
            AppCore::start().await;
        }));
    }

    async fn start() -> Result<(), Box<dyn Error>> {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init();

        let opts = Opts::parse();

        let mut swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_quic()
            .with_dns()?
            .with_relay_client(noise::Config::new, yamux::Config::default)?
            .with_behaviour(|keypair, relay_behaviour| {
                // Define the message ID function for gossipsub
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };

                // Set a custom gossipsub configuration
                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(10))
                    .validation_mode(gossipsub::ValidationMode::Strict)
                    .message_id_fn(message_id_fn)
                    .build()
                    .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?; // Temporary hack because `build` does not return a proper `std::error::Error`.

                // Build the gossipsub behavior
                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(keypair.clone()),
                    gossipsub_config,
                )?;

                Ok(Behaviour {
                    gossipsub,
                    relay_client: relay_behaviour,
                    ping: ping::Behaviour::new(ping::Config::new()),
                    identify: identify::Behaviour::new(identify::Config::new(
                        "/TODO/0.0.1".to_string(),
                        keypair.public(),
                    )),
                    dcutr: dcutr::Behaviour::new(keypair.public().to_peer_id()),
                })
            })?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        // Create a Gossipsub topic
        let topic = gossipsub::IdentTopic::new("test-net");
        // subscribes to our topic
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        // Read full lines from stdin
        let mut stdin = io::BufReader::new(io::stdin()).lines();

        // Listen on all interfaces and whatever port the OS assigns
        swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        println!(
            "Enter messages via STDIN and they will be sent to connected peers using Gossipsub"
        );

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
                    event => panic!("{event:?}"),
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
            loop {
                match swarm.next().await.unwrap() {
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
                        tracing::info!(peer=%peer_id, ?endpoint, "Established new connection. You can now gossip");

                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);

                        loop {
                            select! {
                                Ok(Some(line)) = stdin.next_line() => {
                                    if let Err(e) = swarm
                                        .behaviour_mut().gossipsub
                                        .publish(topic.clone(), line.as_bytes()) {
                                        println!("Publish error: {e:?}");
                                    }
                                }
                                event = swarm.select_next_some() => match event {
                                    SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(gossipsub::Event::Message {
                                        propagation_source: peer_id,
                                        message_id: id,
                                        message,
                                    })) => println!(
                                        "Got message: '{}' with id: {id} from peer: {peer_id}",
                                        String::from_utf8_lossy(&message.data),
                                    ),
                                    SwarmEvent::NewListenAddr { address, .. } => {
                                        println!("Local node is listening on {address}");
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                        tracing::info!(peer=?peer_id, "Outgoing connection failed: {error}");
                    }
                    _ => {}
                }
            }
        })
    }
}

fn generate_ed25519(secret_key_seed: u8) -> identity::Keypair {
    let mut bytes = [0u8; 32];
    bytes[0] = secret_key_seed;

    identity::Keypair::ed25519_from_bytes(bytes).expect("only errors on wrong length")
}
