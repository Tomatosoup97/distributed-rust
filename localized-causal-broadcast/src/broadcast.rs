use crate::network_error::Result as NetworkResult;
use crate::tcp::{Message, TcpHandler};
use crate::udp::Payload;

pub fn best_effort_broadcast(tcp_handler: &TcpHandler, payload: &Payload) -> NetworkResult<()> {
    debug!("Broadcasting: {}", payload);
    for node in tcp_handler.nodes.values() {
        if node.id == tcp_handler.current_node_id {
            continue;
        }
        let message = Message::new(payload.clone(), node.clone());
        tcp_handler.tx_sending_channel.send(message).unwrap();
    }

    tcp_handler.delivered.mark_as_seen(payload)
}

pub fn reliable_broadcast(tcp_handler: &TcpHandler, payload: &Payload) -> NetworkResult<()> {
    if !tcp_handler.delivered.was_seen(payload) {
        best_effort_broadcast(tcp_handler, payload)
    } else {
        Ok(())
    }
}

pub fn uniform_reliable_broadcast(
    tcp_handler: &TcpHandler,
    payload: &Payload,
) -> NetworkResult<()> {
    reliable_broadcast(tcp_handler, payload)
}

pub fn fifo_broadcast(tcp_handler: &TcpHandler, payload: &Payload) -> NetworkResult<()> {
    uniform_reliable_broadcast(tcp_handler, payload)
}

pub fn localized_causal_broadcast(
    tcp_handler: &TcpHandler,
    payload: &Payload,
) -> NetworkResult<()> {
    fifo_broadcast(tcp_handler, payload)
}
