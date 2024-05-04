pub(crate) mod client;
pub(crate) mod server;
pub(crate) mod socket;

pub use client::NetClient;
pub use server::NetServer;
pub use socket::NetSocket;

use crate::util::PixelResBuffer;
use crate::{scene_readers::Scene, util::PixelReqBuffer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum NetCommand {
    Identify,
    ReadScene(Scene),
    #[serde(with = "serde_arrays")]
    RenderPixel(PixelReqBuffer),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NetResponse {
    #[serde(with = "serde_arrays")]
    RenderPixel(PixelResBuffer),
}
