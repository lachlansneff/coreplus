use crate::net::{AsyncRecvFrom, AsyncSendTo, IpAddr, SocketAddr, GetSocketAddrs, MulticastSocket, AddrParseError};
use core::{
    pin::Pin,
    task::{Context, Poll},
};

/// Zero-sized struct that is used for [`GetSocketAddrs`] for std.
///
/// This type is available when the `std` feature is enabled.
#[derive(Clone, Copy, Default)]
pub struct StdGetSocketAddrs;

impl GetSocketAddrs for StdGetSocketAddrs {
    type Iter = std::vec::IntoIter<SocketAddr>;
    type Error = std::io::Error;

    fn get_socket_addrs(&self, host: &str, port: u16) -> Result<Self::Iter, Self::Error> {
        let iter = <(&str, u16) as std::net::ToSocketAddrs>::to_socket_addrs(&(host, port))?;
        Ok(iter.map(Into::<SocketAddr>::into).collect::<Vec<_>>().into_iter())
    }
}

impl From<AddrParseError> for std::io::Error {
    fn from(_: AddrParseError) -> Self {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "address parsing error",
        )
    }
}

impl AsyncSendTo for std::net::UdpSocket {
    type Error = std::io::Error;

    fn poll_send_to(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
        addr: SocketAddr,
    ) -> Poll<Result<usize, std::io::Error>> {
        let addr: std::net::SocketAddr = addr.into();
        Poll::Ready(self.send_to(buf, addr))
    }
}

impl AsyncRecvFrom for std::net::UdpSocket {
    type Error = std::io::Error;

    fn poll_recv_from(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<(usize, SocketAddr), std::io::Error>> {
        Poll::Ready(self.recv_from(buf).map(|(n, addr)| (n, addr.into())))
    }
}

impl MulticastSocket for std::net::UdpSocket {
    type Error = std::io::Error;

    fn join_multicast(&self, addr: IpAddr) -> Result<(), std::io::Error> {
        use std::net::{IpAddr, SocketAddr};
        let local = self.local_addr()?;
        match &(addr.into(), local) {
            (IpAddr::V4(addr), SocketAddr::V4(local)) => self.join_multicast_v4(addr, local.ip()),
            (IpAddr::V4(addr), SocketAddr::V6(local)) => self.join_multicast_v6(&addr.to_ipv6_mapped(), local.scope_id()),
            (IpAddr::V6(addr), SocketAddr::V6(local)) => self.join_multicast_v6(addr, local.scope_id()),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "multicast-addr and local-addr type mismatch",
            ))
        }
    }

    fn leave_multicast(&self, addr: IpAddr) -> Result<(), std::io::Error> {
        use std::net::{IpAddr, SocketAddr};
        let local = self.local_addr()?;
        match &(addr.into(), local) {
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
