pub use no_std_net::{
    IpAddr,
    Ipv4Addr,
    Ipv6Addr,
    SocketAddr,
    SocketAddrV4,
    SocketAddrV6,
};

use heapless::{
    String,
    consts::U256,
};
use core::str::FromStr;

#[derive(Debug)]
pub struct HostAddr {
    ip: IpAddr,
    hostname: Option<String<U256>>,
}

impl HostAddr {
    pub fn new(ip: IpAddr, hostname: Option<String<U256>>) -> Self {
        HostAddr {
            ip,
            hostname,
        }
    }

    pub fn ipv4(octets: [u8;4]) -> HostAddr {
        HostAddr {
            ip: IpAddr::from(octets),
            hostname: None
        }
    }

    pub fn ipv6(octets: [u8;16]) -> HostAddr {
        HostAddr {
            ip: IpAddr::from(octets),
            hostname: None
        }
    }

    pub fn ip(&self) -> IpAddr {
        self.ip
    }

    pub fn hostname(&self) -> Option<&String<U256>> {
        self.hostname.as_ref()
    }
}

#[derive(Debug)]
pub struct AddrParseError;

impl FromStr for HostAddr {
    type Err = AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HostAddr::new(
            IpAddr::from_str(s).map_err(|_| AddrParseError)?,
            Some(String::from_str(s).unwrap()),
        ))
    }
}

impl From<IpAddr> for HostAddr {
    fn from(ip: IpAddr) -> Self {
        HostAddr {
            ip,
            hostname: None,
        }
    }
}

#[derive(Debug)]
pub struct HostSocketAddr {
    addr: HostAddr,
    port: u16,
}

impl HostSocketAddr {
    pub fn new(addr: HostAddr, port: u16) -> HostSocketAddr {
        HostSocketAddr {
            addr,
            port,
        }
    }

    pub fn from(addr: &str, port: u16) -> Result<HostSocketAddr, AddrParseError> {
        Ok( Self::new(
            HostAddr::from_str(addr)?,
            port
        ) )
    }

    pub fn addr(&self) -> &HostAddr {
        &self.addr
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn as_socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.addr.ip, self.port)
    }
}

