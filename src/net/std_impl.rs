use crate::net::{AsyncRecvFrom, AsyncSendTo, IpAddr, NetStack, SocketAddr, ToSocketAddrs, MulticastSocket};
use core::{
    pin::Pin,
    task::{Context, Poll},
};

/// Zero-sized struct that represents the network stack shipped
/// along with the Rust standard library. (e.g. the native network stack)
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct StdNetworking;

impl NetStack for StdNetworking {
    type IpAddr = std::net::IpAddr;

    type SocketAddr = std::net::SocketAddr;

    type Error = std::io::Error;
}

impl<T: std::net::ToSocketAddrs> ToSocketAddrs<StdNetworking> for T {
    type Iter = T::Iter;

    fn to_socket_addrs(&self) -> Result<T::Iter, std::io::Error> {
        self.to_socket_addrs()
    }
}

impl AsyncSendTo<StdNetworking> for std::net::UdpSocket {
    fn poll_send_to(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
        addr: std::net::SocketAddr,
    ) -> Poll<Result<usize, std::io::Error>> {
        Poll::Ready(self.send_to(buf, addr))
    }
}

impl AsyncRecvFrom<StdNetworking> for std::net::UdpSocket {
    fn poll_recv_from(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<(usize, std::net::SocketAddr), std::io::Error>> {
        Poll::Ready(self.recv_from(buf))
    }
}

impl MulticastSocket<StdNetworking> for std::net::UdpSocket {
    fn join_multicast(&self, addr: std::net::IpAddr) -> Result<(), std::io::Error> {
        use std::net::{IpAddr, SocketAddr};
        let local = self.local_addr()?;
        match &(addr, local) {
            (IpAddr::V4(addr), SocketAddr::V4(local)) => self.join_multicast_v4(addr, local.ip()),
            (IpAddr::V4(addr), SocketAddr::V6(local)) => self.join_multicast_v6(&addr.to_ipv6_mapped(), local.scope_id()),
            (IpAddr::V6(addr), SocketAddr::V6(local)) => self.join_multicast_v6(addr, local.scope_id()),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "multicast-addr and local-addr type mismatch",
            ))
        }
    }

    fn leave_multicast(&self, addr: std::net::IpAddr) -> Result<(), std::io::Error> {
        use std::net::{IpAddr, SocketAddr};
        let local = self.local_addr()?;
        match &(addr, local) {
            (IpAddr::V4(addr), SocketAddr::V4(local)) => self.leave_multicast_v4(addr, local.ip()),
            (IpAddr::V4(addr), SocketAddr::V6(local)) => self.leave_multicast_v6(&addr.to_ipv6_mapped(), local.scope_id()),
            (IpAddr::V6(addr), SocketAddr::V6(local)) => self.leave_multicast_v6(addr, local.scope_id()),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "multicast-addr and local-addr type mismatch",
            ))
        }
    }
}

impl IpAddr for std::net::IpAddr {
    fn is_ipv4(&self) -> bool {
        self.is_ipv4()
    }

    fn is_ipv6(&self) -> bool {
        self.is_ipv6()
    }

    fn is_loopback(&self) -> bool {
        self.is_loopback()
    }

    fn is_multicast(&self) -> bool {
        self.is_multicast()
    }

    fn is_unspecified(&self) -> bool {
        self.is_multicast()
    }
}

impl SocketAddr for std::net::SocketAddr {
    type IpAddr = std::net::IpAddr;

    fn new(ip: Self::IpAddr, port: u16) -> Self {
        Self::new(ip, port)
    }

    fn ip(&self) -> Self::IpAddr {
        self.ip()
    }

    fn set_ip(&mut self, new_ip: Self::IpAddr) {
        self.set_ip(new_ip)
    }

    fn port(&self) -> u16 {
        self.port()
    }

    fn set_port(&mut self, new_port: u16) {
        self.set_port(new_port)
    }

    fn is_ipv4(&self) -> bool {
        self.is_ipv4()
    }

    fn is_ipv6(&self) -> bool {
        self.is_ipv6()
    }
}
