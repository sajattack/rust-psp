#![feature(restricted_std)]
#![no_main]

extern crate alloc;

psp::module!("std_udp_test", 1, 0);

#[no_mangle]
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
    }
    let send_buf = b"Hello world!"; 
    let sock = std::net::UdpSocket::bind("10.0.0.215:1337").unwrap();
    sock.send_to(&send_buf[..], "10.0.0.139:34254").unwrap();
    psp::dprintln!("sent Hello world!");
    let mut recv_buf = [0u8; 10];
    sock.recv_from(&mut recv_buf).unwrap();
    let mut text = unsafe { String::from_utf8_unchecked(recv_buf.to_vec()) };
    text = text.replace("\r", "");
    psp::dprintln!("received {}", text);
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
