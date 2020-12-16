
use crate::addr::HostSocketAddr;

use core::fmt::Debug;

/// Whether a socket should block when a read/write can't be performed, or return early.
pub enum Mode {
    /// The function call will wait as long as necessary to complete the operation
    Blocking,
    /// The function call will not wait at all to complete the operation, and only do what it can.
    NonBlocking,
    /// The function call will wait only up the given number of milliseconds to complete the operation.
    Timeout(u16),
}

/// Network errors
#[non_exhaustive]
#[derive(Debug)]
pub enum TcpError {
    NoAvailableSockets,
    ConnectionRefused,
    SocketNotOpen,
    WriteError,
    ReadError,
    Timeout,
    Busy,
    Impl(TcpImplError),
}

#[non_exhaustive]
#[derive(Debug)]
pub enum TcpImplError {
    InitializationError,
    ErrorCode(u32),
    Unknown,
}

/// This trait is implemented by TCP/IP stacks. You could, for example, have an implementation
/// which knows how to send AT commands to an ESP8266 WiFi module. You could have another implemenation
/// which knows how to driver the Rust Standard Library's `std::net` module. Given this trait, you can how
/// write a portable HTTP client which can work with either implementation.
pub trait TcpStack {
    /// The type returned when we create a new TCP socket
    type TcpSocket;
    /// The type returned when we have an error
    type Error: Into<TcpError> + Debug;

    /// Open a new TCP socket. The socket starts in the unconnected state.
    fn open(&self, mode: Mode) -> Result<Self::TcpSocket, Self::Error>;

    /// Connect to the given remote host and port.
    fn connect(
        &self,
        socket: Self::TcpSocket,
        remote: HostSocketAddr,
    ) -> Result<Self::TcpSocket, Self::Error>;

    /// Check if this socket is connected
    fn is_connected(&self, socket: &Self::TcpSocket) -> Result<bool, Self::Error>;

    /// Write to the stream. Returns the number of bytes written is returned
    /// (which may be less than `buffer.len()`), or an error.
    fn write(&self, socket: &mut Self::TcpSocket, buffer: &[u8]) -> nb::Result<usize, Self::Error>;

    /// Read from the stream. Returns `Ok(n)`, which means `n` bytes of
    /// data have been received and they have been placed in
    /// `&buffer[0..n]`, or an error.
    fn read(
        &self,
        socket: &mut Self::TcpSocket,
        buffer: &mut [u8],
    ) -> nb::Result<usize, Self::Error>;

    /// Close an existing TCP socket.
    fn close(&self, socket: Self::TcpSocket) -> Result<(), Self::Error>;
}

