#![no_std]
#![no_main]

#![feature(assoc_char_funcs)]

psp::module!("centronian", 1, 1);

extern crate alloc;

#[allow(bad_style)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_len: u8,
    pub sin_family: u8,
    pub sin_port: u16,
    pub sin_addr: psp::sys::in_addr,
    pub sin_zero: [u8; 8]
}

const AF_INET: u8 = 2;
const SOCK_STREAM: i32 = 1;

fn psp_main() {
    psp::enable_home_button();
    unsafe {
        load_modules();
        init();
        psp::sys::sceNetApctlConnect(1);
        loop {
            let mut state: psp::sys::ApctlState = core::mem::zeroed();
            psp::sys::sceNetApctlGetState(&mut state);
            if let psp::sys::ApctlState::GotIp = state {
                break;
            }
            psp::sys::sceKernelDelayThread(50_000);
        }
        
        let mut rid: i32 = 0;
        let mut buf = [0u8; 1024];
        let mut in_addr: psp::sys::in_addr = core::mem::zeroed();
        psp::sys::sceNetResolverCreate(&mut rid, &mut buf[0] as *mut _ as *mut _, buf.len() as u32);
        psp::sys::sceNetResolverStartNtoA(rid, b"centronian.servebeer.com\0".as_ptr(), &mut in_addr, 5, 5);
        
        let addr_in = sockaddr_in {
            sin_len: 16,
            sin_family: AF_INET,
            sin_port: 6400u16.to_be(),
            sin_addr: in_addr,
            //sin_addr: core::mem::transmute::<[u8;4], psp::sys::in_addr>([205, 250, 172, 72]),
            sin_zero: [0u8; 8]
        };
        
        let sockaddr = core::mem::transmute::<sockaddr_in, psp::sys::sockaddr>(addr_in);
        let sock = psp::sys::sceNetInetSocket(AF_INET as i32, SOCK_STREAM, 0);
        psp::dprintln!("0x{:x}", sock);
        psp::sys::sceNetInetConnect(sock, &sockaddr, core::mem::size_of::<sockaddr_in>() as u32); 
        psp::dprintln!("0x{:x}", psp::sys::sceNetInetGetErrno());
        let mut buf = [0u8; 512];
        psp::sys::sceNetInetRecv(sock, &mut buf[0] as *mut _ as *mut _, 512, 0);
        let mut text = alloc::string::String::from_utf8_unchecked(buf.to_vec());
        text = text.replace("\r", "");
        psp::dprintln!("{}", text);
    }
}

unsafe fn load_modules() {
    psp::sys::sceUtilityLoadNetModule(psp::sys::NetModule::NetCommon);
    psp::sys::sceUtilityLoadNetModule(psp::sys::NetModule::NetInet);
}

unsafe fn init() {
    psp::sys::sceNetInit(0x20000, 0x20, 0x1000, 0x20, 0x1000);
    psp::sys::sceNetInetInit();
    psp::sys::sceNetResolverInit();
    psp::sys::sceNetApctlInit(0x1600, 42);
}
