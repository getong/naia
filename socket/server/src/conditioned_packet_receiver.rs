use std::net::SocketAddr;

use smol::channel::Receiver;

use naia_socket_shared::{link_condition_logic, Instant, LinkConditionerConfig, TimeQueue};

use super::{error::NaiaServerSocketError, packet_receiver::PacketReceiver};

/// Used to receive packets from the Server Socket
#[derive(Clone)]
pub struct ConditionedPacketReceiverImpl {
    #[allow(clippy::type_complexity)]
    channel_receiver: Receiver<Result<(SocketAddr, Box<[u8]>), NaiaServerSocketError>>,
    link_conditioner_config: LinkConditionerConfig,
    time_queue: TimeQueue<(SocketAddr, Box<[u8]>)>,
    last_payload: Option<Box<[u8]>>,
}

impl ConditionedPacketReceiverImpl {
    /// Creates a new PacketReceiver
    #[allow(clippy::type_complexity)]
    pub fn new(
        channel_receiver: Receiver<Result<(SocketAddr, Box<[u8]>), NaiaServerSocketError>>,
        link_conditioner_config: &LinkConditionerConfig,
    ) -> Self {
        Self {
            channel_receiver,
            link_conditioner_config: link_conditioner_config.clone(),
            time_queue: TimeQueue::new(),
            last_payload: None,
        }
    }
}

impl PacketReceiver for ConditionedPacketReceiverImpl {
    fn receive(&mut self) -> Result<Option<(SocketAddr, &[u8])>, NaiaServerSocketError> {
        while let Ok(result) = self.channel_receiver.try_recv() {
            match result {
                Ok(packet) => {
                    link_condition_logic::process_packet(
                        &self.link_conditioner_config,
                        &mut self.time_queue,
                        packet,
                    );
                }
                Err(_) => {
                    break; //TODO: Handle error here
                }
            }
        }

        let now = Instant::now();
        if self.time_queue.has_item(&now) {
            let (address, payload) = self.time_queue.pop_item(&now).unwrap();
            self.last_payload = Some(payload);
            return Ok(Some((address, self.last_payload.as_ref().unwrap())));
        } else {
            Ok(None)
        }
    }
}
