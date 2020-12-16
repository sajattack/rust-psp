#![no_std]
#![no_main]

extern crate alloc;
extern crate libc;

mod ffi;

use core::str::FromStr;

use alloc::alloc::{alloc, Layout};

use drogue_tls::{
    platform::SslPlatform,
    entropy::StaticEntropySource,
    ssl::config::{Preset, Transport, Verify},
    net::tcp_stack::SslTcpStack,
};

use drogue_network::{
    tcp::{Mode, TcpStack},
    addr::HostSocketAddr
};

psp::module!("tls-test", 1, 1);

mod net;

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

    const HEAP_SIZE: usize = 65536;

    let heap = unsafe {
        alloc(Layout::from_size_align(HEAP_SIZE, 4).unwrap())
    };

    let mut ssl_platform = SslPlatform::setup(
        heap as *mut u8 as usize,
    HEAP_SIZE).unwrap();

    ssl_platform.entropy_context_mut().add_source(StaticEntropySource);

    ssl_platform.seed_rng().unwrap();

    let mut ssl_config = ssl_platform.new_client_config(Transport::Stream, Preset::Default).unwrap();
    ssl_config.authmode(Verify::None);
    let network = net::PspTcp::new();
    let secure_network = SslTcpStack::new(ssl_config, &network);

    let socket = secure_network.open(Mode::Blocking).unwrap();
    let socket_addr = HostSocketAddr::from("10.0.0.139", 4443).unwrap();

    psp::dprintln!("attempting connection");
    let mut socket = secure_network.connect(socket, socket_addr).unwrap();
    psp::dprintln!("connected!");

    let result = secure_network.write(&mut socket, b"GET / HTTP/1.1\r\nhost:10.0.0.139\r\n\r\n").unwrap();
    psp::dprintln!("{:?}", result);
    //let result = secure_network.read(&mut buf,
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
