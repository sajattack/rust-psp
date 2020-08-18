#![feature(restricted_std)]
#![no_main]

use std::io::{Read, Write};

psp::module!("std_centronian", 1, 1);

#[no_mangle]
fn psp_main() {
    psp::enable_home_button();
    unsafe {
        panic!();
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

        let mut stream: std::net::TcpStream;
        let stream_result = std::net::TcpStream::connect("centronian.servebeer.com:6400"); 
        if stream_result.is_ok() {
            stream = stream_result.unwrap();
            psp::dprintln!("{}", stream.peer_addr().unwrap());
        } else {
            psp::dprintln!("{}", stream_result.unwrap_err());
            panic!();
        }
        let mut buf = [0u8; 512];
        stream.read(&mut buf).unwrap();
        let mut text = String::from_utf8_unchecked(buf.to_vec());
        text = text.replace("\r", "");
        psp::dprintln!("{}", text);

        stream.write(b"e\r\n").unwrap();
        let mut buf = [0u8; 512];
        stream.read(&mut buf).unwrap();
        let mut text = String::from_utf8_unchecked(buf.to_vec());
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
