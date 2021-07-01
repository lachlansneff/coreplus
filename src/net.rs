//! Networking traits

use core::{
    pin::Pin,
    task::{Context, Poll},
};

mod addr;
mod ip;
mod parser;
#[cfg(feature = "std")]
mod std_impl;

pub use self::addr::{GetSocketAddrs, SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs};
pub use self::ip::{IpAddr, Ipv4Addr, Ipv6Addr, Ipv6MulticastScope};
pub use self::parser::AddrParseError;
#[cfg(feature = "std")]
pub use self::std_impl::*;

/// This is intended for async datagram IO.
pub trait AsyncSendTo {
    /// The associated error type.
    type Error;

    /// A non-blocking, poll-based variant of [`std::net::UdpSocket::send_to`].
    ///
    /// This doesn't _have_ to be implemented as async. The implementation
    /// for [`std::net::UdpSocket`] is blocking.
    fn poll_send_to(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
        addr: SocketAddr,
    ) -> Poll<Result<usize, Self::Error>>;
}

/// This is intended for async datagram IO.
pub trait AsyncRecvFrom {
    /// The associated error type.
    type Error;

    /// A non-blocking, poll-based varient of [`std::net::UdpSocket::recv_from`].
    ///
    /// This doesn't _have_ to be implemented as async. The implementation
    /// for [`std::net::UdpSocket`] is blocking.
    fn poll_recv_from(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<(usize, SocketAddr), Self::Error>>;
}

pub trait MulticastSocket {
    /// The associated error type.
    type Error;

    /// Attempts to join the given multicast group.
    fn join_multicast(&self, addr: IpAddr) -> Result<(), Self::Error>;

    /// Attempts to leave the given multicast group.
    fn leave_multicast(&self, addr: IpAddr) -> Result<(), Self::Error>;
}
