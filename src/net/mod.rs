pub(crate) mod client;
pub(crate) mod server;
pub(crate) mod socket;

pub use client::NetClient;
pub use server::NetServer;
pub use socket::NetSocket;

use crate::{scene_readers::Scene, util::PixelReq};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum NetCommand {
    Identify,
    ReadScene(Scene),
    RenderPixel(PixelReq),
}
