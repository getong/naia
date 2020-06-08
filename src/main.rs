
#[macro_use]
extern crate log;

use simple_logger;

use gaia_server::{GaiaServer, ServerEvent, find_my_ip_address, Config};

use gaia_example_shared::{event_manifest_load, entity_manifest_load, StringEvent, PointEntity, ExampleEvent, ExampleEntity};

use std::{
    rc::Rc,
    net::SocketAddr,
    time::Duration};

const SERVER_PORT: &str = "3179";

#[tokio::main]
async fn main() {

    simple_logger::init_with_level(log::Level::Info).expect("A logger was already initialized");

    let current_socket_address = find_my_ip_address::get() + ":" + SERVER_PORT;

    let mut config = Config::default();
    config.tick_interval = Duration::from_secs(2);
    config.heartbeat_interval = Duration::from_secs(4);

    let mut server = GaiaServer::listen(current_socket_address.as_str(),
                                        event_manifest_load(),
                                        entity_manifest_load(),
                                        Some(config)).await;

    let mut point_entity = PointEntity::new(0, 0);
    server.add_entity(point_entity.clone());

    server.on_scope_entity(Rc::new(Box::new(|address, entity| {
        match entity {
            ExampleEntity::PointEntity(point_entity) => {
                let x = point_entity.get_x();
                return x >= 3 && x <= 7;
            }
        }
    })));

    loop {
        match server.receive().await {
            Ok(event) => {
                match event {
                    ServerEvent::Connection(address) => {
                        info!("Gaia Server connected to: {}", address);
                    }
                    ServerEvent::Disconnection(address) => {
                        info!("Gaia Server disconnected from: {:?}", address);
                    }
                    ServerEvent::Event(address, event_type) => {
                        match event_type {
                            ExampleEvent::StringEvent(string_event) => {
                                let message = string_event.get_message();
                                match message {
                                    Some(msg) => {
                                        info!("Gaia Server recv <- {}: {}", address, msg);
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                    ServerEvent::Tick => {
                        // This could be used for your non-network logic (game loop?)

                        // Event Sending
//                        for addr in server.get_clients() {
//                            let count = server.get_sequence_number(addr).expect("why don't we have a sequence number for this client?");
//                            let new_message = "Server Packet (".to_string() + count.to_string().as_str() + ") to " + addr.to_string().as_str();
//                            info!("Gaia Server send -> {}: {}", addr, new_message);
//
//                            let string_event = StringEvent::new(new_message);
//                            server.send_event(addr, &string_event);
//                        }

                        point_entity.borrow_mut().step();
                    }
                }
            }
            Err(error) => {
                info!("Gaia Server Error: {}", error);
            }
        }
    }
}