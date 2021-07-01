//! Networking traits

use core::{
    fmt,
    hash::Hash,
    pin::Pin,
    str::FromStr,
    task::{Context, Poll},
};

#[cfg(feature = "std")]
mod std_impl;

#[cfg(feature = "std")]
pub use self::std_impl::*;

/// A networking stack.
///
/// This is the "base" trait that is implemented for network stacks.
/// This trait is implemented for `StdNetworking`
/// when the `std` feature is enabled.
pub trait NetStack {
    /// An IP address
    type IpAddr: IpAddr;
    /// An IP address and port
    type SocketAddr: SocketAddr<IpAddr = Self::IpAddr>;
    /// Error type
    type Error;
}

/// An IP address.
///
/// This trait is implemented for [`std::net::IpAddr`] when the `std` feature is enabled.
pub trait IpAddr:
    FromStr
    + Hash
    + Ord
    + PartialOrd
    + PartialEq
    + Eq
    + Copy
    + Clone
    + fmt::Debug
    + fmt::Display
    + From<[u8; 4]>
    + From<[u8; 16]>
    + From<[u16; 8]>
{
    fn is_ipv4(&self) -> bool;
    fn is_ipv6(&self) -> bool;
    fn is_loopback(&self) -> bool;
    fn is_multicast(&self) -> bool;
    fn is_unspecified(&self) -> bool;
}

/// An internet socket address.
///
/// This trait is implemented for [`std::net::SocketAddr`] when the `std` feature is enabled.
pub trait SocketAddr:
    FromStr + Hash + Ord + PartialEq + PartialOrd + Eq + Copy + Clone + fmt::Debug + fmt::Display
{
    type IpAddr: IpAddr;

    fn new(ip: Self::IpAddr, port: u16) -> Self;
    fn ip(&self) -> Self::IpAddr;
    fn set_ip(&mut self, new_ip: Self::IpAddr);
    fn port(&self) -> u16;
    fn set_port(&mut self, new_port: u16);
    fn is_ipv4(&self) -> bool;
    fn is_ipv6(&self) -> bool;
}

/// Retrive the addresses associated with a hostname.
///
/// When implementing this for your network stack, it's  recommended to add
/// the following implementations:
/// ```rust
/// impl ToSocketAddrs<YourNetworkStack> for YourSocketAddr { ... }
/// impl ToSocketAddrs<YourNetworkStack> for (&str, u16) { ... }
/// impl ToSocketAddrs<YourNetworkStack> for (YourIpAddr, u16) { ... }
/// impl ToSocketAddrs<YourNetworkStack> for &str { ... }
/// ```
pub trait ToSocketAddrs<N: NetStack> {
    /// An iterator over addresses + port.
    type Iter: Iterator<Item = N::SocketAddr>;

    /// Converts this object to an iterator of resolved `SocketAddr`s.
    ///
    /// The returned iterator may not actually yield any values depending on the
    /// outcome of any resolution performed.
    ///
    /// Note that this function may block the current thread while resolution is
    /// performed.
    fn to_socket_addrs(&self) -> Result<Self::Iter, N::Error>;
}

/// This is intended for async datagram IO.
pub trait AsyncSendTo<N: NetStack> {
    /// A non-blocking, poll-based variant of [`std::net::UdpSocket::send_to`].
    ///
    /// This doesn't _have_ to be implemented as async. The implementation
    /// for [`StdNetworking`] is blocking.
    fn poll_send_to(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
        addr: N::SocketAddr,
    ) -> Poll<Result<usize, N::Error>>;
}

/// This is intended for async datagram IO.
pub trait AsyncRecvFrom<N: NetStack> {
    /// A non-blocking, poll-based varient of [`std::net::UdpSocket::recv_from`].
    ///
    /// This doesn't _have_ to be implemented as async. The implementation
    /// for [`StdNetworking`] is blocking.
    fn poll_recv_from(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<(usize, N::SocketAddr), N::Error>>;
}

pub trait MulticastSocket<N: NetStack> {
    /// Attempts to join the given multicast group.
    fn join_multicast(&self, addr: N::IpAddr) -> Result<(), N::Error>;

    /// Attempts to leave the given multicast group.
    fn leave_multicast(&self, addr: N::IpAddr) -> Result<(), N::Error>;
}
