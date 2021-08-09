use psp::sys;
use drogue_tls::TlsError;
use drogue_network::addr::*;

use core::ffi::c_void;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Socket(i32);


impl Socket {
    pub fn open() -> Result<Socket,()> {
        let sock = unsafe { sys::sceNetInetSocket(netc::AF_INET as i32, netc::SOCK_STREAM, 0) };
        if sock < 0 {
            Err(())
        } else {
            Ok(Socket(sock))
        }
    }

    pub fn connect(&self, remote: HostSocketAddr) -> Result<(), ()> {
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

                if unsafe { sys::sceNetInetConnect(self.0, &sockaddr, core::mem::size_of::<netc::sockaddr_in>() as u32) } < 0  {
                    unsafe { psp::dprintln!("0x{:08x}", sys::sceNetInetGetErrno()); }
                    Err(())
                } else {
                    Ok(())
                }
            }
            SocketAddr::V6(_) => {
                Err(())
            }
        }
    }

    fn _read(self, buf: &mut [u8]) -> Result<usize, ()> {
        let result = unsafe { sys::sceNetInetRecv(self.0, buf.as_mut_ptr() as *mut c_void, buf.len(), 0) };
        if (result as i32) < 0 {
            Err(())
        } else {
            Ok(result as usize)
        }
    }

    fn _write(&self, buf: &[u8]) -> Result<usize, ()> {
        let result = unsafe { sys::sceNetInetSend(self.0, buf.as_ptr() as *const c_void, buf.len(), 0) };
        if (result as i32) < 0 {
            Err(())
        } else {
            Ok(result as usize)
        }
    }
}

impl drogue_tls::traits::Read for Socket {
    fn read<'m>(&'m mut self, buf: &'m mut [u8]) -> Result<usize, TlsError> {
        self._read(buf).map_err(|_| TlsError::InternalError)
    }
}

impl drogue_tls::traits::Write for Socket {
    fn write<'m>(&'m mut self, buf: &'m [u8]) -> Result<usize, TlsError> {
        self._write(buf).map_err(|_| TlsError::InternalError)
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
