# Limiinal - Planning

## Project Overview

- Cross-platform peer to peer Rust client for Limiinal.
- Using the libp2p network library.

## Features We Want

- List of core features (essential).
  - Hole Punching [docs](https://docs.rs/libp2p/latest/libp2p/tutorials/hole_punching/index.html)
- Optional/nice-to-have features (stretch goals).
  - Send pings encrypted with noise or alternatives
  - Light and dark mode gui

## Tech Stack

- Rust Programming Language
- libp2p for secure peer to peer networking
- iced-rs

## Design and Architecture

- High-level diagrams or flowcharts (can be added later).
- How things work together

- Early UI design idea:

![image](https://github.com/user-attachments/assets/e1f64535-618a-43bb-a63a-690bdc580641)


## To Do List

- Major milestones with rough timelines.
  - Milestone 1: Basic peer-to-peer communication.
  - Milestone 2: User interface prototype.
    - [ ] Windows
    - [ ] Linux
    - [ ] MacOS
    - [ ] Mobile?

## Task Breakdown

- High-level tasks for each milestone.

  - Important Concepts

    - Transports [docs](https://docs.libp2p.io/concepts/transports/overview/)
    - DoS Mitigation [docs](https://docs.libp2p.io/concepts/security/dos-mitigation/#incorporating-dos-mitigation-from-the-start)
    - Swarm [docs](https://docs.rs/libp2p/latest/libp2p/swarm/index.html)
    - PeerId to identify other nodes [docs](https://docs.rs/libp2p/latest/libp2p/struct.PeerId.html)
    - Holepunching [example](https://github.com/libp2p/rust-libp2p/tree/master/examples/dcutr)

  - Handling Input
  - Sending Messages
  - Responding to Messages

- Who does what.
- Prioritization of tasks (e.g., P1 - Critical, P2 - Important).

## Potential Challenges

## References and Resources

- Documentation links (e.g., libp2p, Rust docs).
- [Async Rust Book](https://rust-lang.github.io/async-book/)
- [libp2p Rust Docs](https://docs.rs/libp2p/latest/libp2p/tutorials/index.html#modules)
- [libp2p Docs](https://docs.libp2p.io/)
- [libp2p Main Site](https://libp2p.io/)
- **[Really useful getting started with rust rust p2p guide](https://docs.rs/libp2p/latest/libp2p/tutorials/ping/index.html)**
- [iced-rs for gui](https://book.iced.rs/)

## TODO List

- [x] Research libp2p documentation and examples.
- [x] Set up the project repository and initialize with Cargo.
- [ ] Implement basic peer-to-peer connection.
- [ ] Test peer-to-peer messaging functionality.
- [ ] Design initial UI/UX mockups for the chat interface.
- [ ] Integrate encryption for secure communication.
- [ ] Develop error-handling mechanisms for peer disconnections.
- [ ] Write unit tests for core functionality.
- [ ] Plan stretch features (e.g., group chats, file sharing).
- [ ] Update documentation as the project evolves.
