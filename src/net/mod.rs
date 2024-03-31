pub(crate) mod client;
pub(crate) mod server;
pub(crate) mod socket;

pub use client::NetClient;
pub use server::NetServer;
pub use socket::NetSocket;

use serde::{Deserialize, Serialize};
use crate::scene_readers::Scene;

#[derive(Serialize, Deserialize, Debug)]
pub enum NetCommand {
    Identify,
    ReadScene(Scene),
    RenderPixel((usize, usize)),
}

