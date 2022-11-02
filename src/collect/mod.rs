use std::net::UdpSocket;

pub struct SipCollector {
    socket: UdpSocket,
    raw_byte: Vec<u8>,
}