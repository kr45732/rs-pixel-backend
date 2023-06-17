use crate::structs::WebError;
use actix_web::HttpResponse;
use rs_pixel::{util::error::Error, HypixelEndpoint};
use serde::Serialize;
use std::sync::Arc;

pub const RESOURCES: [HypixelEndpoint; 12] = [
    HypixelEndpoint::RESOURCES_GAMES,
    HypixelEndpoint::RESOURCES_ACHIEVEMENTS,
    HypixelEndpoint::RESOURCES_CHALLENGES,
    HypixelEndpoint::RESOURCES_QUESTS,
    HypixelEndpoint::RESOURCES_GUILD_ACHIEVEMENTS,
    HypixelEndpoint::RESOURCES_VANITY_PETS,
    HypixelEndpoint::RESOURCES_VANITY_COMPANIONS,
    HypixelEndpoint::RESOURCES_SKYBLOCK_COLLECTIONS,
    HypixelEndpoint::RESOURCES_SKYBLOCK_SKILLS,
    HypixelEndpoint::RESOURCES_SKYBLOCK_ITEMS,
    HypixelEndpoint::RESOURCES_SKYBLOCK_ELECTION,
    HypixelEndpoint::RESOURCES_SKYBLOCK_BINGO,
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
