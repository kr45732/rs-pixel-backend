mod endpoints;
mod structs;
mod utils;

use crate::{
    endpoints::{
        boosters, counts, guild, leaderboards, player, punishment_stats, recent_games, resources,
        skyblock_auction, skyblock_auctions, skyblock_auctions_ended, skyblock_bazaar,
        skyblock_bingo, skyblock_fire_sales, skyblock_news, skyblock_profile, skyblock_profiles,
        status,
    },
    structs::WebData,
};
use actix_web::{
    web::{redirect, resource, Data},
    App, HttpServer,
};
use dotenv::dotenv;
use rs_pixel::{util::minecraft::ApiType, ConfigBuilder, HypixelEndpoint, RsPixel};
use std::{env, error::Error, time::Duration};
use tokio::sync::Mutex;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Reading config...");
    if dotenv().is_err() {
        println!("Cannot find a .env file, will attempt to use environment variables");
    }

    let api_key = env::var("API_KEY")
        .unwrap_or_else(|_| panic!("Unable to find API_KEY environment variable"));
    let port = env::var("PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or_else(|| panic!("Unable to find PORT environment variable"));
    let base_url = env::var("BASE_URL")
        .unwrap_or_else(|_| panic!("Unable to find BASE_URL environment variable"));

    let config = ConfigBuilder::default()
        .client(
            surf::Config::new()
                .set_timeout(Some(Duration::from_secs(15)))
                .try_into()?,
        )
        .minecraft_api_type(ApiType::PlayerDb)
        .minecraft_cache_ttl(Duration::from_secs(900))
        .add_hypixel_cache_ttl(HypixelEndpoint::KEY, Duration::from_secs(60))
        .add_hypixel_cache_ttl(HypixelEndpoint::BOOSTERS, Duration::from_secs(60))
        .add_hypixel_cache_ttl(HypixelEndpoint::LEADERBOARDS, Duration::from_secs(60))
        .add_hypixel_cache_ttl(HypixelEndpoint::PUNISHMENT_STATS, Duration::from_secs(60))
        .add_hypixel_cache_ttl(HypixelEndpoint::PLAYER, Duration::from_secs(60))
        .add_hypixel_cache_ttl(HypixelEndpoint::GUILD, Duration::from_secs(60))
        .add_hypixel_cache_ttl(HypixelEndpoint::COUNTS, Duration::from_secs(60))
        .add_hypixel_cache_ttl(HypixelEndpoint::STATUS, Duration::from_secs(60))
        .add_hypixel_cache_ttl(HypixelEndpoint::RECENT_GAMES, Duration::from_secs(60))
        .add_hypixel_cache_ttl(HypixelEndpoint::SKYBLOCK_PROFILES, Duration::from_secs(90))
        .add_hypixel_cache_ttl(HypixelEndpoint::SKYBLOCK_PROFILE, Duration::from_secs(90))
        .add_hypixel_cache_ttl(HypixelEndpoint::SKYBLOCK_BINGO, Duration::from_secs(60))
        .add_hypixel_cache_ttl(HypixelEndpoint::SKYBLOCK_NEWS, Duration::from_secs(60))
        .add_hypixel_cache_ttl(HypixelEndpoint::SKYBLOCK_AUCTION, Duration::from_secs(60))
        .add_hypixel_cache_ttl(HypixelEndpoint::SKYBLOCK_AUCTIONS, Duration::from_secs(60))
        .add_hypixel_cache_ttl(
            HypixelEndpoint::SKYBLOCK_AUCTIONS_ENDED,
            Duration::from_secs(60),
        )
        .add_hypixel_cache_ttl(HypixelEndpoint::SKYBLOCK_BAZAAR, Duration::from_secs(60))
        .add_hypixel_cache_ttl(HypixelEndpoint::SKYBLOCK_FIRESALES, Duration::from_secs(60))
        .add_hypixel_cache_ttl(HypixelEndpoint::RESOURCES_GAMES, Duration::from_secs(900))
        .add_hypixel_cache_ttl(
            HypixelEndpoint::RESOURCES_ACHIEVEMENTS,
            Duration::from_secs(900),
        )
        .add_hypixel_cache_ttl(
            HypixelEndpoint::RESOURCES_CHALLENGES,
            Duration::from_secs(900),
        )
        .add_hypixel_cache_ttl(HypixelEndpoint::RESOURCES_QUESTS, Duration::from_secs(900))
        .add_hypixel_cache_ttl(
            HypixelEndpoint::RESOURCES_GUILD_ACHIEVEMENTS,
            Duration::from_secs(900),
        )
        .add_hypixel_cache_ttl(
            HypixelEndpoint::RESOURCES_VANITY_PETS,
            Duration::from_secs(900),
        )
        .add_hypixel_cache_ttl(
            HypixelEndpoint::RESOURCES_VANITY_COMPANIONS,
            Duration::from_secs(900),
        )
        .add_hypixel_cache_ttl(
            HypixelEndpoint::RESOURCES_SKYBLOCK_COLLECTIONS,
            Duration::from_secs(900),
        )
        .add_hypixel_cache_ttl(
            HypixelEndpoint::RESOURCES_SKYBLOCK_SKILLS,
            Duration::from_secs(900),
        )
        .add_hypixel_cache_ttl(
            HypixelEndpoint::RESOURCES_SKYBLOCK_ITEMS,
            Duration::from_secs(900),
        )
        .add_hypixel_cache_ttl(
            HypixelEndpoint::RESOURCES_SKYBLOCK_ELECTION,
            Duration::from_secs(900),
        )
        .add_hypixel_cache_ttl(
            HypixelEndpoint::RESOURCES_SKYBLOCK_BINGO,
            Duration::from_secs(900),
        )
        .into();

    let web_data = Data::new(WebData {
        api: Mutex::new(RsPixel::from_config(api_key, config).await?),
    });

    println!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .app_data(web_data.clone())
            .service(redirect("/", "https://github.com/kr45732/rs-pixel"))
            // .service(key)
            .service(boosters)
            .service(leaderboards)
            .service(punishment_stats)
            .service(player)
            .service(guild)
            .service(counts)
            .service(status)
            .service(recent_games)
            .service(skyblock_profiles)
            .service(skyblock_profile)
            .service(skyblock_bingo)
            .service(skyblock_news)
            .service(skyblock_auction)
            .service(skyblock_auctions)
            .service(skyblock_auctions_ended)
            .service(skyblock_bazaar)
            .service(skyblock_fire_sales)
            .service(
                resource([
                    "resources",
                    "resources/{resource}",
                    "resources/{resource}/{sub_resource}",
                ])
                .to(resources),
            )
    })
    .bind((base_url, port))?
    .run()
    .await?;

    Ok(())
}
