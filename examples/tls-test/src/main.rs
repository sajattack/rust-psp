#![no_std]
#![no_main]

extern crate alloc;

use psp::sys;

use drogue_tls::blocking::*;
use drogue_network::addr::HostSocketAddr;
use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::SeedableRng;

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
    
    let socket = net::Socket::open().expect("failed to open socket");
    socket.connect(HostSocketAddr::from("93.184.216.34", 443).expect("failed to create address")).expect("failed to connect socket");
    let mut seed: u64 = 0;
    unsafe { 
        sys::sceRtcGetCurrentTick(&mut seed as *mut u64);
    }
    let rng = ChaCha20Rng::seed_from_u64(seed);

    let mut record_buffer = [0; 16384];
    let tls_context = TlsContext::new(rng, &mut record_buffer).with_server_name("example.com");
    let mut tls: TlsConnection<ChaCha20Rng, net::Socket, Aes128GcmSha256> =
        TlsConnection::new(tls_context, socket);

    tls.open().expect("error establishing TLS connection");

    tls.write(b"GET /a/check HTTP/1.1\r\nHost: www.example.com\r\nUser-Agent: A fucking PSP!\r\n\r\n").expect("error writing data");


    let mut rx_buf = [0; 4096];
    let sz = tls.read(&mut rx_buf).expect("error reading data");
    unsafe {
        let mut text = alloc::string::String::from_utf8_unchecked(rx_buf.to_vec());
        text = text.replace("\r","");
        text = text.replace("\0","");
        psp::dprintln!("Read {} bytes: {}", sz, text);
    }

    let mut rx_buf = [0; 4096];
    let sz = tls.read(&mut rx_buf).expect("error reading data");
    unsafe {
        let mut text = alloc::string::String::from_utf8_unchecked(rx_buf.to_vec());
        text = text.replace("\r","");
        text = text.replace("\0","");
        psp::dprintln!("Read {} bytes: {}", sz, text);
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
