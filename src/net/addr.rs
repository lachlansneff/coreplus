use core::{fmt, option, str};

use crate::{io::Write, net::{IpAddr, Ipv4Addr, Ipv6Addr, AddrParseError}};


/// An internet socket address, either IPv4 or IPv6.
///
/// Internet socket addresses consist of an [IP address], a 16-bit port number, as well
/// as possibly some version-dependent additional information. See [`SocketAddrV4`]'s and
/// [`SocketAddrV6`]'s respective documentation for more details.
///
/// The size of a `SocketAddr` instance may vary depending on the target operating
/// system.
///
/// [IP address]: IpAddr
///
/// # Examples
///
/// ```
/// use coreplus::net::{IpAddr, Ipv4Addr, SocketAddr};
///
/// let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
///
/// assert_eq!("127.0.0.1:8080".parse(), Ok(socket));
/// assert_eq!(socket.port(), 8080);
/// assert_eq!(socket.is_ipv4(), true);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SocketAddr {
    /// An IPv4 socket address.
    V4(SocketAddrV4),
    /// An IPv6 socket address.
    V6(SocketAddrV6),
}

/// An IPv4 socket address.
///
/// IPv4 socket addresses consist of an [`IPv4` address] and a 16-bit port number, as
/// stated in [IETF RFC 793].
///
/// See [`SocketAddr`] for a type encompassing both IPv4 and IPv6 socket addresses.
///
/// The size of a `SocketAddrV4` struct may vary depending on the target operating
/// system. Do not assume that this type has the same memory layout as the underlying
/// system representation.
///
/// [IETF RFC 793]: https://tools.ietf.org/html/rfc793
/// [`IPv4` address]: Ipv4Addr
///
/// # Examples
///
/// ```
/// use coreplus::net::{Ipv4Addr, SocketAddrV4};
///
/// let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
///
/// assert_eq!("127.0.0.1:8080".parse(), Ok(socket));
/// assert_eq!(socket.ip(), &Ipv4Addr::new(127, 0, 0, 1));
/// assert_eq!(socket.port(), 8080);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketAddrV4 {
    // Do not assume that this struct is implemented as the underlying system representation.
    // The memory layout is not part of the stable interface that std exposes.
    ip: Ipv4Addr,
    port: u16,
}

/// An IPv6 socket address.
///
/// IPv6 socket addresses consist of an [`IPv6` address], a 16-bit port number, as well
/// as fields containing the traffic class, the flow label, and a scope identifier
/// (see [IETF RFC 2553, Section 3.3] for more details).
///
/// See [`SocketAddr`] for a type encompassing both IPv4 and IPv6 socket addresses.
///
/// The size of a `SocketAddrV6` struct may vary depending on the target operating
/// system. Do not assume that this type has the same memory layout as the underlying
/// system representation.
///
/// [IETF RFC 2553, Section 3.3]: https://tools.ietf.org/html/rfc2553#section-3.3
/// [`IPv6` address]: Ipv6Addr
///
/// # Examples
///
/// ```
/// use coreplus::net::{Ipv6Addr, SocketAddrV6};
///
/// let socket = SocketAddrV6::new(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
///
/// assert_eq!("[2001:db8::1]:8080".parse(), Ok(socket));
/// assert_eq!(socket.ip(), &Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
/// assert_eq!(socket.port(), 8080);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketAddrV6 {
    // Do not assume that this struct is implemented as the underlying system representation.
    // The memory layout is not part of the stable interface that std exposes.
    ip: Ipv6Addr,
    port: u16,
    flowinfo: u32,
    scope_id: u32,
}

impl SocketAddr {
    /// Creates a new socket address from an [IP address] and a port number.
    ///
    /// [IP address]: IpAddr
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{IpAddr, Ipv4Addr, SocketAddr};
    ///
    /// let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    /// assert_eq!(socket.port(), 8080);
    /// ```
    pub fn new(ip: IpAddr, port: u16) -> SocketAddr {
        match ip {
            IpAddr::V4(a) => SocketAddr::V4(SocketAddrV4::new(a, port)),
            IpAddr::V6(a) => SocketAddr::V6(SocketAddrV6::new(a, port, 0, 0)),
        }
    }

    /// Returns the IP address associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{IpAddr, Ipv4Addr, SocketAddr};
    ///
    /// let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    /// ```
    pub const fn ip(&self) -> IpAddr {
        match *self {
            SocketAddr::V4(ref a) => IpAddr::V4(*a.ip()),
            SocketAddr::V6(ref a) => IpAddr::V6(*a.ip()),
        }
    }

    /// Changes the IP address associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{IpAddr, Ipv4Addr, SocketAddr};
    ///
    /// let mut socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// socket.set_ip(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)));
    /// assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)));
    /// ```
    pub fn set_ip(&mut self, new_ip: IpAddr) {
        // `match (*self, new_ip)` would have us mutate a copy of self only to throw it away.
        match (self, new_ip) {
            (&mut SocketAddr::V4(ref mut a), IpAddr::V4(new_ip)) => a.set_ip(new_ip),
            (&mut SocketAddr::V6(ref mut a), IpAddr::V6(new_ip)) => a.set_ip(new_ip),
            (self_, new_ip) => *self_ = Self::new(new_ip, self_.port()),
        }
    }

    /// Returns the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{IpAddr, Ipv4Addr, SocketAddr};
    ///
    /// let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// assert_eq!(socket.port(), 8080);
    /// ```
    pub const fn port(&self) -> u16 {
        match *self {
            SocketAddr::V4(ref a) => a.port(),
            SocketAddr::V6(ref a) => a.port(),
        }
    }

    /// Changes the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{IpAddr, Ipv4Addr, SocketAddr};
    ///
    /// let mut socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// socket.set_port(1025);
    /// assert_eq!(socket.port(), 1025);
    /// ```
    pub fn set_port(&mut self, new_port: u16) {
        match *self {
            SocketAddr::V4(ref mut a) => a.set_port(new_port),
            SocketAddr::V6(ref mut a) => a.set_port(new_port),
        }
    }

    /// Returns [`true`] if the [IP address] in this `SocketAddr` is an
    /// [`IPv4` address], and [`false`] otherwise.
    ///
    /// [IP address]: IpAddr
    /// [`IPv4` address]: IpAddr::V4
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{IpAddr, Ipv4Addr, SocketAddr};
    ///
    /// let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// assert_eq!(socket.is_ipv4(), true);
    /// assert_eq!(socket.is_ipv6(), false);
    /// ```
    pub const fn is_ipv4(&self) -> bool {
        matches!(*self, SocketAddr::V4(_))
    }

    /// Returns [`true`] if the [IP address] in this `SocketAddr` is an
    /// [`IPv6` address], and [`false`] otherwise.
    ///
    /// [IP address]: IpAddr
    /// [`IPv6` address]: IpAddr::V6
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{IpAddr, Ipv6Addr, SocketAddr};
    ///
    /// let socket = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 1)), 8080);
    /// assert_eq!(socket.is_ipv4(), false);
    /// assert_eq!(socket.is_ipv6(), true);
    /// ```
    pub const fn is_ipv6(&self) -> bool {
        matches!(*self, SocketAddr::V6(_))
    }
}

impl SocketAddrV4 {
    /// Creates a new socket address from an [`IPv4` address] and a port number.
    ///
    /// [`IPv4` address]: Ipv4Addr
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{SocketAddrV4, Ipv4Addr};
    ///
    /// let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    /// ```
    pub fn new(ip: Ipv4Addr, port: u16) -> SocketAddrV4 {
        SocketAddrV4 {
            ip,
            port,
        }
    }

    /// Returns the IP address associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{SocketAddrV4, Ipv4Addr};
    ///
    /// let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    /// assert_eq!(socket.ip(), &Ipv4Addr::new(127, 0, 0, 1));
    /// ```
    pub const fn ip(&self) -> &Ipv4Addr {
        &self.ip
    }

    /// Changes the IP address associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{SocketAddrV4, Ipv4Addr};
    ///
    /// let mut socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    /// socket.set_ip(Ipv4Addr::new(192, 168, 0, 1));
    /// assert_eq!(socket.ip(), &Ipv4Addr::new(192, 168, 0, 1));
    /// ```
    pub fn set_ip(&mut self, new_ip: Ipv4Addr) {
        self.ip = new_ip
    }

    /// Returns the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{SocketAddrV4, Ipv4Addr};
    ///
    /// let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    /// assert_eq!(socket.port(), 8080);
    /// ```
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// Changes the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{SocketAddrV4, Ipv4Addr};
    ///
    /// let mut socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    /// socket.set_port(4242);
    /// assert_eq!(socket.port(), 4242);
    /// ```
    pub fn set_port(&mut self, new_port: u16) {
        self.port = new_port
    }
}

impl SocketAddrV6 {
    /// Creates a new socket address from an [`IPv6` address], a 16-bit port number,
    /// and the `flowinfo` and `scope_id` fields.
    ///
    /// For more information on the meaning and layout of the `flowinfo` and `scope_id`
    /// parameters, see [IETF RFC 2553, Section 3.3].
    ///
    /// [IETF RFC 2553, Section 3.3]: https://tools.ietf.org/html/rfc2553#section-3.3
    /// [`IPv6` address]: Ipv6Addr
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    /// ```
    pub fn new(ip: Ipv6Addr, port: u16, flowinfo: u32, scope_id: u32) -> SocketAddrV6 {
        SocketAddrV6 {
            ip,
            port,
            flowinfo,
            scope_id,
        }
    }

    /// Returns the IP address associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    /// assert_eq!(socket.ip(), &Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    /// ```
    pub const fn ip(&self) -> &Ipv6Addr {
        &self.ip
    }

    /// Changes the IP address associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    /// socket.set_ip(Ipv6Addr::new(76, 45, 0, 0, 0, 0, 0, 0));
    /// assert_eq!(socket.ip(), &Ipv6Addr::new(76, 45, 0, 0, 0, 0, 0, 0));
    /// ```
    pub fn set_ip(&mut self, new_ip: Ipv6Addr) {
        self.ip = new_ip
    }

    /// Returns the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    /// assert_eq!(socket.port(), 8080);
    /// ```
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// Changes the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    /// socket.set_port(4242);
    /// assert_eq!(socket.port(), 4242);
    /// ```
    pub fn set_port(&mut self, new_port: u16) {
        self.port = new_port
    }

    /// Returns the flow information associated with this address.
    ///
    /// This information corresponds to the `sin6_flowinfo` field in C's `netinet/in.h`,
    /// as specified in [IETF RFC 2553, Section 3.3].
    /// It combines information about the flow label and the traffic class as specified
    /// in [IETF RFC 2460], respectively [Section 6] and [Section 7].
    ///
    /// [IETF RFC 2553, Section 3.3]: https://tools.ietf.org/html/rfc2553#section-3.3
    /// [IETF RFC 2460]: https://tools.ietf.org/html/rfc2460
    /// [Section 6]: https://tools.ietf.org/html/rfc2460#section-6
    /// [Section 7]: https://tools.ietf.org/html/rfc2460#section-7
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 10, 0);
    /// assert_eq!(socket.flowinfo(), 10);
    /// ```
    pub const fn flowinfo(&self) -> u32 {
        self.flowinfo
    }

    /// Changes the flow information associated with this socket address.
    ///
    /// See [`SocketAddrV6::flowinfo`]'s documentation for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 10, 0);
    /// socket.set_flowinfo(56);
    /// assert_eq!(socket.flowinfo(), 56);
    /// ```
    pub fn set_flowinfo(&mut self, new_flowinfo: u32) {
        self.flowinfo = new_flowinfo
    }

    /// Returns the scope ID associated with this address.
    ///
    /// This information corresponds to the `sin6_scope_id` field in C's `netinet/in.h`,
    /// as specified in [IETF RFC 2553, Section 3.3].
    ///
    /// [IETF RFC 2553, Section 3.3]: https://tools.ietf.org/html/rfc2553#section-3.3
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 78);
    /// assert_eq!(socket.scope_id(), 78);
    /// ```
    pub const fn scope_id(&self) -> u32 {
        self.scope_id
    }

    /// Changes the scope ID associated with this socket address.
    ///
    /// See [`SocketAddrV6::scope_id`]'s documentation for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use coreplus::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 78);
    /// socket.set_scope_id(42);
    /// assert_eq!(socket.scope_id(), 42);
    /// ```
    pub fn set_scope_id(&mut self, new_scope_id: u32) {
        self.scope_id = new_scope_id
    }
}

impl From<SocketAddrV4> for SocketAddr {
    /// Converts a [`SocketAddrV4`] into a [`SocketAddr::V4`].
    fn from(sock4: SocketAddrV4) -> SocketAddr {
        SocketAddr::V4(sock4)
    }
}

impl From<SocketAddrV6> for SocketAddr {
    /// Converts a [`SocketAddrV6`] into a [`SocketAddr::V6`].
    fn from(sock6: SocketAddrV6) -> SocketAddr {
        SocketAddr::V6(sock6)
    }
}

impl<I: Into<IpAddr>> From<(I, u16)> for SocketAddr {
    /// Converts a tuple struct (Into<[`IpAddr`]>, `u16`) into a [`SocketAddr`].
    ///
    /// This conversion creates a [`SocketAddr::V4`] for a [`IpAddr::V4`]
    /// and creates a [`SocketAddr::V6`] for a [`IpAddr::V6`].
    ///
    /// `u16` is treated as port of the newly created [`SocketAddr`].
    fn from(pieces: (I, u16)) -> SocketAddr {
        SocketAddr::new(pieces.0.into(), pieces.1)
    }
}

impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SocketAddr::V4(ref a) => a.fmt(f),
            SocketAddr::V6(ref a) => a.fmt(f),
        }
    }
}

impl fmt::Debug for SocketAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl fmt::Display for SocketAddrV4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Fast path: if there's no alignment stuff, write to the output buffer
        // directly
        if f.precision().is_none() && f.width().is_none() {
            write!(f, "{}:{}", self.ip(), self.port())
        } else {
            const IPV4_SOCKET_BUF_LEN: usize = (3 * 4)  // the segments
                + 3  // the separators
                + 1 + 5; // the port
            let mut buf = [0; IPV4_SOCKET_BUF_LEN];
            let mut buf_slice = &mut buf[..];

            // Unwrap is fine because writing to a sufficiently-sized
            // buffer is infallible
            write!(buf_slice, "{}:{}", self.ip(), self.port()).unwrap();
            let len = IPV4_SOCKET_BUF_LEN - buf_slice.len();

            // This unsafe is OK because we know what is being written to the buffer
            let buf = unsafe { str::from_utf8_unchecked(&buf[..len]) };
            f.pad(buf)
        }
    }
}

impl fmt::Debug for SocketAddrV4 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl fmt::Display for SocketAddrV6 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Fast path: if there's no alignment stuff, write to the output
        // buffer directly
        if f.precision().is_none() && f.width().is_none() {
            match self.scope_id() {
                0 => write!(f, "[{}]:{}", self.ip(), self.port()),
                scope_id => write!(f, "[{}%{}]:{}", self.ip(), scope_id, self.port()),
            }
        } else {
            const IPV6_SOCKET_BUF_LEN: usize = (4 * 8)  // The address
            + 7  // The colon separators
            + 2  // The brackets
            + 1 + 10 // The scope id
            + 1 + 5; // The port

            let mut buf = [0; IPV6_SOCKET_BUF_LEN];
            let mut buf_slice = &mut buf[..];

            match self.scope_id() {
                0 => write!(buf_slice, "[{}]:{}", self.ip(), self.port()),
                scope_id => write!(buf_slice, "[{}%{}]:{}", self.ip(), scope_id, self.port()),
            }
            // Unwrap is fine because writing to a sufficiently-sized
            // buffer is infallible
            .unwrap();
            let len = IPV6_SOCKET_BUF_LEN - buf_slice.len();

            // This unsafe is OK because we know what is being written to the buffer
            let buf = unsafe { str::from_utf8_unchecked(&buf[..len]) };
            f.pad(buf)
        }
    }
}

impl fmt::Debug for SocketAddrV6 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

/// Do a DNS query to lookup ip addresses associated with a hostname.
///
/// This is blocking.
pub trait GetSocketAddrs {
    type Iter: Iterator<Item = SocketAddr>;
    type Error: From<AddrParseError>;

    fn get_socket_addrs(&self, host: &str, port: u16) -> Result<Self::Iter, Self::Error>;
}

pub enum OneOrMany<I: Iterator> {
    One(option::IntoIter<I::Item>),
    Many(I),
}

impl<I: Iterator> OneOrMany<I> {
    pub fn one(item: I::Item) -> Self {
        Self::One(Some(item).into_iter())
    }
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
pub trait ToSocketAddrs<T: GetSocketAddrs> {
    /// Converts this object to an iterator of resolved `SocketAddr`s.
    ///
    /// The returned iterator may not actually yield any values depending on the
    /// outcome of any resolution performed.
    ///
    /// Note that this function may block the current thread while resolution is
    /// performed.
    fn to_socket_addrs(&self, get: &T) -> Result<OneOrMany<T::Iter>, T::Error>;
}

impl<T: GetSocketAddrs> ToSocketAddrs<T> for SocketAddr {
    fn to_socket_addrs(&self, _get: &T) -> Result<OneOrMany<T::Iter>, T::Error> {
        Ok(OneOrMany::one(*self))
    }
}

impl<T: GetSocketAddrs> ToSocketAddrs<T> for SocketAddrV4 {
    fn to_socket_addrs(&self, get: &T) -> Result<OneOrMany<T::Iter>, T::Error> {
        SocketAddr::V4(*self).to_socket_addrs(get)
    }
}

impl<T: GetSocketAddrs> ToSocketAddrs<T> for SocketAddrV6 {
    fn to_socket_addrs(&self, get: &T) -> Result<OneOrMany<T::Iter>, T::Error> {
        SocketAddr::V6(*self).to_socket_addrs(get)
    }
}

impl<T: GetSocketAddrs> ToSocketAddrs<T> for (IpAddr, u16) {
    fn to_socket_addrs(&self, get: &T) -> Result<OneOrMany<T::Iter>, T::Error> {
        let (ip, port) = *self;
        match ip {
            IpAddr::V4(ref a) => (*a, port).to_socket_addrs(get),
            IpAddr::V6(ref a) => (*a, port).to_socket_addrs(get),
        }
    }
}

impl<T: GetSocketAddrs> ToSocketAddrs<T> for (Ipv4Addr, u16) {
    fn to_socket_addrs(&self, get: &T) -> Result<OneOrMany<T::Iter>, T::Error> {
        let (ip, port) = *self;
        SocketAddrV4::new(ip, port).to_socket_addrs(get)
    }
}

impl<T: GetSocketAddrs> ToSocketAddrs<T> for (Ipv6Addr, u16) {
    fn to_socket_addrs(&self, get: &T) -> Result<OneOrMany<T::Iter>, T::Error> {
        let (ip, port) = *self;
        SocketAddrV6::new(ip, port, 0, 0).to_socket_addrs(get)
    }
}

impl<T: GetSocketAddrs> ToSocketAddrs<T> for (&str, u16) {
    fn to_socket_addrs(&self, get: &T) -> Result<OneOrMany<T::Iter>, T::Error> {
        let (host, port) = *self;

        // try to parse the host as a regular IP address first
        if let Ok(addr) = host.parse::<Ipv4Addr>() {
            let addr = SocketAddrV4::new(addr, port);
            return Ok(OneOrMany::one(SocketAddr::V4(addr)));
        }
        if let Ok(addr) = host.parse::<Ipv6Addr>() {
            let addr = SocketAddrV6::new(addr, port, 0, 0);
            return Ok(OneOrMany::one(SocketAddr::V6(addr)));
        }

        get.get_socket_addrs(host, port).map(|iter| OneOrMany::Many(iter))
    }
}

// TODO: alloc feature?
// impl<T: GetSocketAddrs> ToSocketAddrs<T> for (String, u16) {
//     fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<SocketAddr>> {
//         (&*self.0, self.1).to_socket_addrs()
//     }
// }

// accepts strings like 'localhost:12345'
impl<T: GetSocketAddrs> ToSocketAddrs<T> for str {
    fn to_socket_addrs(&self, get: &T) -> Result<OneOrMany<T::Iter>, T::Error> {
        // try to parse as a regular SocketAddr first
        if let Ok(addr) = self.parse() {
            return Ok(OneOrMany::one(addr));
        }

        let (host, port_str) = self.rsplit_once(':').ok_or(AddrParseError(()))?;
        let port: u16 = port_str.parse().map_err(|_| AddrParseError(()))?;

        get.get_socket_addrs(host, port).map(|iter| OneOrMany::Many(iter))
    }
}

// #[stable(feature = "slice_to_socket_addrs", since = "1.8.0")]
// impl<'a> ToSocketAddrs for &'a [SocketAddr] {
//     type Iter = iter::Cloned<slice::Iter<'a, SocketAddr>>;

//     fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
//         Ok(self.iter().cloned())
//     }
// }

impl<T: GetSocketAddrs, U: ToSocketAddrs<T> + ?Sized> ToSocketAddrs<T> for &U {
    fn to_socket_addrs(&self, get: &T) -> Result<OneOrMany<T::Iter>, T::Error> {
        (**self).to_socket_addrs(get)
    }
}

// TODO: alloc feature?
// #[stable(feature = "string_to_socket_addrs", since = "1.16.0")]
// impl ToSocketAddrs for String {
//     type Iter = vec::IntoIter<SocketAddr>;
//     fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<SocketAddr>> {
//         (&**self).to_socket_addrs()
//     }
// }

#[cfg(feature = "std")]
impl From<std::net::SocketAddrV4> for SocketAddrV4 {
    fn from(addr: std::net::SocketAddrV4) -> Self {
        Self::new((*addr.ip()).into(), addr.port())
    }
}

#[cfg(feature = "std")]
impl From<std::net::SocketAddrV4> for SocketAddr {
    fn from(addr: std::net::SocketAddrV4) -> Self {
        let a: SocketAddrV4 = addr.into();
        a.into()
    }
}

#[cfg(feature = "std")]
impl From<std::net::SocketAddrV6> for SocketAddrV6 {
    fn from(addr: std::net::SocketAddrV6) -> Self {
        Self::new((*addr.ip()).into(), addr.port(), addr.flowinfo(), addr.scope_id())
    }
}

#[cfg(feature = "std")]
impl From<std::net::SocketAddrV6> for SocketAddr {
    fn from(addr: std::net::SocketAddrV6) -> Self {
        let a: SocketAddrV6 = addr.into();
        a.into()
    }
}

#[cfg(feature = "std")]
impl From<std::net::SocketAddr> for SocketAddr {
    fn from(addr: std::net::SocketAddr) -> Self {
        match addr {
            std::net::SocketAddr::V4(ip) => ip.into(),
            std::net::SocketAddr::V6(ip) => ip.into(),
        }
    }
}

#[cfg(feature = "std")]
impl From<SocketAddrV4> for std::net::SocketAddrV4 {
    fn from(addr: SocketAddrV4) -> Self {
        Self::new((*addr.ip()).into(), addr.port())
    }
}

#[cfg(feature = "std")]
impl From<SocketAddrV4> for std::net::SocketAddr {
    fn from(addr: SocketAddrV4) -> Self {
        let a: std::net::SocketAddrV4 = addr.into();
        a.into()
    }
}

#[cfg(feature = "std")]
impl From<SocketAddrV6> for std::net::SocketAddrV6 {
    fn from(addr: SocketAddrV6) -> Self {
        Self::new((*addr.ip()).into(), addr.port(), addr.flowinfo(), addr.scope_id())
    }
}

#[cfg(feature = "std")]
impl From<SocketAddrV6> for std::net::SocketAddr {
    fn from(addr: SocketAddrV6) -> Self {
        let a: std::net::SocketAddrV6 = addr.into();
        a.into()
    }
}

#[cfg(feature = "std")]
impl From<SocketAddr> for std::net::SocketAddr {
    fn from(addr: SocketAddr) -> Self {
        match addr {
            SocketAddr::V4(ip) => ip.into(),
            SocketAddr::V6(ip) => ip.into(),
        }
    }
}