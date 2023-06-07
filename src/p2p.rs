// ████████ ██   ██  █████  ██       █████  ███    ███ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ████  ████ ██    ██ ██      
//    ██    ███████ ███████ ██      ███████ ██ ████ ██ ██    ██ ███████ 
//    ██    ██   ██ ██   ██ ██      ██   ██ ██  ██  ██ ██    ██      ██ 
//    ██    ██   ██ ██   ██ ███████ ██   ██ ██      ██  ██████  ███████                                                                             
// Copyright 2021-2023 The Open Sam Foundation (OSF)
// Developed by Caleb Mitchell Smith (PixelCoda)
// Licensed under GPLv3....see LICENSE file.

use std::error::Error;

use libp2p::futures::StreamExt;
// use std::io::Result;




// use futures::StreamExt;
use libp2p::{
    core::transport::upgrade::Version,
    multiaddr::Protocol,
    identify, identity, noise, ping, rendezvous,
    swarm::{keep_alive, NetworkBehaviour, SwarmBuilder, SwarmEvent},
    tcp, yamux, PeerId, Transport, Multiaddr,
};
use std::time::Duration;

pub async fn init_p2p_server() -> Result<(), Box<dyn Error>> {
    // env_logger::init();

    let key_pair = identity::Keypair::generate_ed25519();

    log::info!("identity generated");

    let mut swarm = SwarmBuilder::with_tokio_executor(
        tcp::tokio::Transport::default()
            .upgrade(Version::V1Lazy)
            .authenticate(noise::Config::new(&key_pair).unwrap())
            .multiplex(yamux::Config::default())
            .boxed(),
        ServerBehaviour {
            identify: identify::Behaviour::new(identify::Config::new(
                "rendezvous-example/1.0.0".to_string(),
                key_pair.public(),
            )),
            rendezvous: rendezvous::server::Behaviour::new(rendezvous::server::Config::default()),
            ping: ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(1))),
            keep_alive: keep_alive::Behaviour,
        },
        PeerId::from(key_pair.public()),
    )
    .build();

    log::warn!("SERVER_ID: {}", swarm.local_peer_id());

    let _ = swarm.listen_on("/ip4/0.0.0.0/tcp/62649".parse().unwrap());

    while let Some(event) = swarm.next().await {
        match event {
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                log::warn!("Connected to {}", peer_id);
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                log::warn!("Disconnected from {}", peer_id);
            }
            SwarmEvent::Behaviour(ServerBehaviourEvent::Rendezvous(
                rendezvous::server::Event::PeerRegistered { peer, registration },
            )) => {
                log::warn!(
                    "Peer {} registered for namespace '{}'",
                    peer,
                    registration.namespace
                );
            }
            SwarmEvent::Behaviour(ServerBehaviourEvent::Ping(ping::Event {
                peer,
                result: Ok(rtt),
                ..
            })) => {
                match rtt {
                    libp2p::ping::Success::Ping{rtt: stt} => {
                        log::warn!("Server Ping to {} in {:?}", peer, stt);
                    },
                    libp2p::ping::Success::Pong{} => {
                        log::warn!("Server Pong from {}", peer);
                    }
                }
                
            }
            SwarmEvent::Behaviour(ServerBehaviourEvent::Rendezvous(
                rendezvous::server::Event::DiscoverServed {
                    enquirer,
                    registrations,
                },
            )) => {
                log::warn!(
                    "Served peer {} with {} registrations",
                    enquirer,
                    registrations.len()
                );
            }
            other => {
                log::debug!("Unhandled {:?}", other);
            }
        }
    }

    Ok(())
}

const NAMESPACE: &str = "rendezvous";

pub async fn init_p2p_client(server_ip_address: String) -> Result<(), Box<dyn Error>> {
    // env_logger::init();

    let key_pair = identity::Keypair::generate_ed25519();
    let rendezvous_point_address = format!("/ip4/{}/tcp/62649", server_ip_address).as_str().parse::<Multiaddr>().unwrap();
    // let rendezvous_point = "12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN".parse().unwrap();

    let mut swarm = SwarmBuilder::with_tokio_executor(
        tcp::tokio::Transport::default()
            .upgrade(Version::V1Lazy)
            .authenticate(noise::Config::new(&key_pair).unwrap())
            .multiplex(yamux::Config::default())
            .boxed(),
        MyBehaviour {
            identify: identify::Behaviour::new(identify::Config::new(
                "rendezvous-example/1.0.0".to_string(),
                key_pair.public(),
            )),
            rendezvous: rendezvous::client::Behaviour::new(key_pair.clone()),
            ping: ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(1))),
            keep_alive: keep_alive::Behaviour,
        },
        PeerId::from(key_pair.public()),
    )
    .build();

    let external_address = format!("/ip4/{}/tcp/62649", server_ip_address).as_str().parse::<Multiaddr>().unwrap();
    swarm.add_external_address(external_address, libp2p::swarm::AddressScore::Infinite);

    log::warn!("Local peer id: {}", swarm.local_peer_id());

    swarm.dial(rendezvous_point_address.clone()).unwrap();

    let mut discover_tick = tokio::time::interval(Duration::from_secs(30));
    let mut cookie = None;

    loop {
        tokio::select! {
                event = swarm.select_next_some() => match event {
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        log::warn!(
                            "Connected to rendezvous point, discovering nodes in '{}' namespace ...",
                            NAMESPACE
                        );

                        swarm.behaviour_mut().rendezvous.discover(
                            Some(rendezvous::Namespace::new(NAMESPACE.to_string()).unwrap()),
                            None,
                            None,
                            peer_id,
                        );

                        swarm.behaviour_mut().rendezvous.register(
                            rendezvous::Namespace::from_static("rendezvous"),
                            peer_id,
                            None,
                        );

                        log::warn!("Connection established with rendezvous point {}", peer_id);
                    }
                    SwarmEvent::Behaviour(MyBehaviourEvent::Rendezvous(rendezvous::client::Event::Discovered {
                        registrations,
                        cookie: new_cookie,
                        ..
                    })) => {
                        cookie.replace(new_cookie);

                        for registration in registrations {
                            for address in registration.record.addresses() {
                                let peer = registration.record.peer_id();
                                log::warn!("Discovered peer {} at {}", peer, address);

                                let p2p_suffix = Protocol::P2p(*peer.as_ref());
                                let address_with_p2p =
                                    if !address.ends_with(&Multiaddr::empty().with(p2p_suffix.clone())) {
                                        address.clone().with(p2p_suffix)
                                    } else {
                                        address.clone()
                                    };

                                swarm.dial(address_with_p2p).unwrap();
                            }
                        }
                    }
                    SwarmEvent::Behaviour(MyBehaviourEvent::Identify(identify::Event::Received {
                        peer_id, info, ..
                    })) => {
                        log::warn!("{peer_id:?}: {info:?}");
                 
                    }
                    SwarmEvent::Behaviour(MyBehaviourEvent::Identify(identify::Event::Sent {
                        peer_id, ..
                    })) => {
                        log::warn!("Sent identify info to {peer_id:?}");
                        swarm.behaviour_mut().rendezvous.register(
                            rendezvous::Namespace::from_static("rendezvous"),
                            peer_id,
                            None,
                        );
                    }
    
                    SwarmEvent::Behaviour(MyBehaviourEvent::Rendezvous(
                        rendezvous::client::Event::Registered {
                            namespace,
                            ttl,
                            rendezvous_node,
                        },
                    )) => {
                        log::warn!(
                            "Registered for namespace '{}' at rendezvous point {} for the next {} seconds",
                            namespace,
                            rendezvous_node,
                            ttl
                        );
                    }
                    SwarmEvent::Behaviour(MyBehaviourEvent::Rendezvous(
                        rendezvous::client::Event::RegisterFailed(error),
                    )) => {
                        log::error!("Failed to register {}", error);
                    }

                    SwarmEvent::Behaviour(MyBehaviourEvent::Ping(ping::Event {
                        peer,
                        result: Ok(rtt),
                        ..
                    })) => {
                        match rtt {
                            libp2p::ping::Success::Ping{rtt: stt} => {
                                log::warn!("Ping to {} in {:?}", peer, stt);
                            },
                            libp2p::ping::Success::Pong{} => {

                            }
                        }
                        
                    }
                    other => {
                        log::debug!("Unhandled {:?}", other);
                    }
            },
            _ = discover_tick.tick(), if cookie.is_some() => {}
                // swarm.behaviour_mut().rendezvous.discover(
                //     Some(rendezvous::Namespace::new(NAMESPACE.to_string()).unwrap()),
                //     cookie.clone(),
                //     None,
                //     rendezvous_point)
        }
    }
}

pub async fn exp() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    log::warn!("Local peer id: {local_peer_id:?}");

    let transport = tcp::async_io::Transport::default()
        .upgrade(Version::V1Lazy)
        .authenticate(noise::Config::new(&local_key).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();

    // Create a identify network behaviour.
    let behaviour = identify::Behaviour::new(identify::Config::new(
        "/ipfs/id/1.0.0".to_string(),
        local_key.public(),
    ));

    let mut swarm =
        SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build();

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        log::warn!("Dialed {addr}")
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => log::warn!("Listening on {address:?}"),
            // Prints peer id identify info is being sent to.
            SwarmEvent::Behaviour(identify::Event::Sent { peer_id, .. }) => {
                log::warn!("Sent identify info to {peer_id:?}")
            }
            // Prints out the info received via the identify event
            SwarmEvent::Behaviour(identify::Event::Received { info, .. }) => {
                log::warn!("Received {info:?}")
            }
            _ => {}
        }
    }
}


#[derive(NetworkBehaviour)]
struct ServerBehaviour {
    identify: identify::Behaviour,
    rendezvous: rendezvous::server::Behaviour,
    ping: ping::Behaviour,
    keep_alive: keep_alive::Behaviour,
}

#[derive(NetworkBehaviour)]
struct MyBehaviour {
    identify: identify::Behaviour,
    rendezvous: rendezvous::client::Behaviour,
    ping: ping::Behaviour,
    keep_alive: keep_alive::Behaviour,
}
