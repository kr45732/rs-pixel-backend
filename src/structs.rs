use rs_pixel::RsPixel;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

pub struct WebData {
    pub api: Mutex<RsPixel>,
}

#[derive(Serialize)]
pub struct WebError {
    pub success: bool,
    pub cause: String,
}

#[derive(Deserialize)]
pub struct PlayerQuery {
    pub username: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Deserialize)]
pub struct GuildQuery {
    pub id: Option<String>,
    pub name: Option<String>,
    pub player: Option<String>,
    pub username: Option<String>,
}

#[derive(Deserialize)]
pub struct StatusQuery {
    pub username: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Deserialize)]
pub struct RecentGamesQuery {
    pub username: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Deserialize)]
pub struct SkyblockProfilesQuery {
    pub username: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Deserialize)]
pub struct SkyblockProfileQuery {
    pub profile: Option<String>,
}

#[derive(Deserialize)]
pub struct SkyblockBingoQuery {
    pub username: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Deserialize)]
pub struct AuctionQuery {
    pub player: Option<String>,
    pub uuid: Option<String>,
    pub profile: Option<String>,
    pub username: Option<String>,
}

#[derive(Deserialize)]
pub struct SkyblockAuctionQuery {
    pub page: Option<i64>,
}

#[derive(Deserialize)]
pub struct ResourcesPath {
    pub resource: Option<String>,
    pub sub_resource: Option<String>,
}
