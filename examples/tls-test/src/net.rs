use psp::sys;
use nb;
use drogue_network::{
    tcp::{TcpError, TcpStack, Mode},
    addr::{HostSocketAddr, SocketAddr},
};

use core::ffi::c_void;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Socket(i32);

pub struct PspTcp();

impl PspTcp {
    pub fn new() -> Self {
        Self()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PspTcpError {
    Unknown,
}

impl From<PspTcpError> for TcpError {
    fn from(err: PspTcpError) -> TcpError {
        TcpError::Impl(drogue_network::tcp::TcpImplError::Unknown)
    }
}

impl From<TcpError> for PspTcpError {
    fn from(err: TcpError) -> PspTcpError {
        PspTcpError::Unknown
    }
}

static mut CONNECTED: bool = false;

impl TcpStack for PspTcp {
    type TcpSocket = Socket;
    type Error = PspTcpError;

    fn open(&self, mode: Mode) -> Result<Self::TcpSocket, Self::Error> {
        let sock = unsafe { sys::sceNetInetSocket(netc::AF_INET as i32, netc::SOCK_STREAM, 0) };
        if sock < 0 {
            Err(PspTcpError::Unknown)
        } else {
            Ok(Socket(sock))
        }
    }

    fn connect(&self, socket: Self::TcpSocket, remote: HostSocketAddr) -> Result<Self::TcpSocket, Self::Error> {
        match remote.as_socket_addr() {
            SocketAddr::V4(v4) => {
                let octets = v4.ip().octets();
                let sin_addr = u32::from_le_bytes(octets);
                let port = v4.port().to_be();

                let sockaddr_in = netc::sockaddr_in {
                    sin_len: core::mem::size_of::<netc::sockaddr_in>() as u8,
                    sin_family: netc::AF_INET,
                    sin_port: port,
                    sin_addr: netc::in_addr(sin_addr),
                    sin_zero: [0u8; 8],
                };

                let sockaddr = unsafe { core::mem::transmute::<netc::sockaddr_in, netc::sockaddr>(sockaddr_in) };

                if unsafe { sys::sceNetInetConnect(socket.0, &sockaddr, core::mem::size_of::<netc::sockaddr_in>() as u32) } < 0  {
                    unsafe { psp::dprintln!("0x{:08x}", sys::sceNetInetGetErrno()); }
                    Err(PspTcpError::Unknown)
                } else {
                    unsafe { CONNECTED = true; }
                    Ok(Socket(socket.0))
                }
            }
            SocketAddr::V6(_) => {
                Err(PspTcpError::Unknown)
            }
        }
    }

    fn read(&self, socket: &mut Self::TcpSocket, buf: &mut [u8]) -> Result<usize, nb::Error<Self::Error>> {
        let result = unsafe { sys::sceNetInetRecv(socket.0, buf.as_mut_ptr() as *mut c_void, buf.len(), 0) };
        if (result as i32) < 0 {
            Err(nb::Error::Other(PspTcpError::Unknown))
        } else {
            Ok(result as usize)
        }
    }

    fn write(&self, socket: &mut Self::TcpSocket, buf: &[u8]) -> Result<usize, nb::Error<Self::Error>> {
        let result = unsafe { sys::sceNetInetSend(socket.0, buf.as_ptr() as *const c_void, buf.len(), 0) };
        if (result as i32) < 0 {
            Err(nb::Error::Other(PspTcpError::Unknown))
        } else {
            Ok(result as usize)
        }
    }

    fn close(&self, socket: Self::TcpSocket) -> Result<(), Self::Error> {
        unsafe { sys::sceNetInetClose(socket.0); }
        unsafe { CONNECTED = false; }
        Ok(())
    }

    fn is_connected(&self, socket: &Self::TcpSocket) -> Result<bool, Self::Error> {
        Ok(unsafe { CONNECTED })
    }
}


#[allow(nonstandard_style)]
pub mod netc {
    pub const AF_INET: u8 = 2;

    pub const SOCK_STREAM: i32 = 1;

    pub type sa_family_t = u8;

    pub use psp::sys::in_addr;

    pub use psp::sys::sockaddr;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sockaddr_in {
        pub sin_len: u8,
        pub sin_family: u8,
        pub sin_port: u16,
        pub sin_addr: in_addr,
        pub sin_zero: [u8; 8]
    }
}
