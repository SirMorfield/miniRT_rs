pub(crate) mod client;
pub(crate) mod server;
pub(crate) mod socket;

pub use client::NetClient;
pub use server::NetServer;
pub use socket::NetSocket;