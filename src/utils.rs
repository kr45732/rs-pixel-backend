use crate::structs::WebError;
use actix_web::HttpResponse;
use rs_pixel::{util::error::Error, HypixelEndpoint};
use serde::Serialize;
use std::sync::Arc;

pub const SERVER_ENDPOINTS: [&str; 19] = [
    "KEY",
    "BOOSTERS",
    "LEADERBOARDS",
    "PUNISHMENT_STATS",
    "PLAYER",
    "GUILD",
    "COUNTS",
    "STATUS",
    "RECENT_GAMES",
    "SKYBLOCK_PROFILES",
    "SKYBLOCK_PROFILE",
    "SKYBLOCK_BINGO",
    "SKYBLOCK_NEWS",
    "SKYBLOCK_AUCTION",
    "SKYBLOCK_AUCTIONS",
    "SKYBLOCK_AUCTIONS_ENDED",
    "SKYBLOCK_BAZAAR",
    "SKYBLOCK_FIRESALES",
    "RESOURCES",
];

pub const HYPIXEL_ENDPOINTS: [(&str, HypixelEndpoint, bool); 30] = [
    ("KEY", HypixelEndpoint::KEY, false),
    ("BOOSTERS", HypixelEndpoint::BOOSTERS, false),
    ("LEADERBOARDS", HypixelEndpoint::LEADERBOARDS, false),
    ("PUNISHMENT_STATS", HypixelEndpoint::PUNISHMENT_STATS, false),
    ("PLAYER", HypixelEndpoint::PLAYER, false),
    ("GUILD", HypixelEndpoint::GUILD, false),
    ("COUNTS", HypixelEndpoint::COUNTS, false),
    ("STATUS", HypixelEndpoint::STATUS, false),
    ("RECENT_GAMES", HypixelEndpoint::RECENT_GAMES, false),
    (
        "SKYBLOCK_PROFILES",
        HypixelEndpoint::SKYBLOCK_PROFILES,
        false,
    ),
    ("SKYBLOCK_PROFILE", HypixelEndpoint::SKYBLOCK_PROFILE, false),
    ("SKYBLOCK_BINGO", HypixelEndpoint::SKYBLOCK_BINGO, false),
    ("SKYBLOCK_NEWS", HypixelEndpoint::SKYBLOCK_NEWS, false),
    ("SKYBLOCK_AUCTION", HypixelEndpoint::SKYBLOCK_AUCTION, false),
    (
        "SKYBLOCK_AUCTIONS",
        HypixelEndpoint::SKYBLOCK_AUCTIONS,
        false,
    ),
    (
        "SKYBLOCK_AUCTIONS_ENDED",
        HypixelEndpoint::SKYBLOCK_AUCTIONS_ENDED,
        false,
    ),
    ("SKYBLOCK_BAZAAR", HypixelEndpoint::SKYBLOCK_BAZAAR, false),
    (
        "SKYBLOCK_FIRESALES",
        HypixelEndpoint::SKYBLOCK_FIRESALES,
        false,
    ),
    ("RESOURCES_GAMES", HypixelEndpoint::RESOURCES_GAMES, true),
    (
        "RESOURCES_ACHIEVEMENTS",
        HypixelEndpoint::RESOURCES_ACHIEVEMENTS,
        true,
    ),
    (
        "RESOURCES_CHALLENGES",
        HypixelEndpoint::RESOURCES_CHALLENGES,
        true,
    ),
    ("RESOURCES_QUESTS", HypixelEndpoint::RESOURCES_QUESTS, true),
    (
        "RESOURCES_GUILD_ACHIEVEMENTS",
        HypixelEndpoint::RESOURCES_GUILD_ACHIEVEMENTS,
        true,
    ),
    (
        "RESOURCES_VANITY_PETS",
        HypixelEndpoint::RESOURCES_VANITY_PETS,
        true,
    ),
    (
        "RESOURCES_VANITY_COMPANIONS",
        HypixelEndpoint::RESOURCES_VANITY_COMPANIONS,
        true,
    ),
    (
        "RESOURCES_SKYBLOCK_COLLECTIONS",
        HypixelEndpoint::RESOURCES_SKYBLOCK_COLLECTIONS,
        true,
    ),
    (
        "RESOURCES_SKYBLOCK_SKILLS",
        HypixelEndpoint::RESOURCES_SKYBLOCK_SKILLS,
        true,
    ),
    (
        "RESOURCES_SKYBLOCK_ITEMS",
        HypixelEndpoint::RESOURCES_SKYBLOCK_ITEMS,
        true,
    ),
    (
        "RESOURCES_SKYBLOCK_ELECTION",
        HypixelEndpoint::RESOURCES_SKYBLOCK_ELECTION,
        true,
    ),
    (
        "RESOURCES_SKYBLOCK_BINGO",
        HypixelEndpoint::RESOURCES_SKYBLOCK_BINGO,
        true,
    ),
];

pub fn ok<T>(value: Arc<T>) -> HttpResponse
where
    T: Serialize,
{
    HttpResponse::Ok().json(&*value)
}

pub fn error_response(err: Error) -> HttpResponse {
    HttpResponse::BadRequest().json(WebError {
        success: false,
        cause: err.to_string(),
    })
}

pub fn bad_request(cause: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(WebError {
        success: false,
        cause: cause.to_string(),
    })
}
