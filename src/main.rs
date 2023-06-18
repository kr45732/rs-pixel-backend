mod endpoints;
mod structs;
mod utils;

use crate::{
    endpoints::{add_endpoint, default},
    structs::WebData,
    utils::{HYPIXEL_ENDPOINTS, SERVER_ENDPOINTS},
};
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use dotenv::dotenv;
use rs_pixel::{util::minecraft::ApiType, ConfigBuilder, RateLimitStrategy, RsPixel};
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

    let mut config = ConfigBuilder::default().client(
        surf::Config::new()
            .set_timeout(Some(Duration::from_secs(15)))
            .try_into()?,
    );

    if let Ok(minecraft_api_type) = env::var("MINECRAFT_API_TYPE") {
        config = config.minecraft_api_type(match minecraft_api_type.as_str() {
            "Mojang" => ApiType::Mojang,
            "Ashcon" => ApiType::Ashcon,
            "PlayerDb" => ApiType::PlayerDb,
            _ => panic!("Unable to parse MINECRAFT_API_TYPE environment variable"),
        });
    }

    if let Ok(rate_limit_strategy) = env::var("RATE_LIMIT_STRATEGY") {
        config = config.rate_limit_strategy(match rate_limit_strategy.as_str() {
            "Delay" => RateLimitStrategy::Delay,
            "Error" => RateLimitStrategy::Error,
            _ => panic!("Unable to parse RATE_LIMIT_STRATEGY environment variable"),
        });
    }

    if let Ok(minecraft_cache_ttl) = env::var("MINECRAFT_CACHE_TTL") {
        match minecraft_cache_ttl.parse::<u64>() {
            Ok(v) => {
                config = config.minecraft_cache_ttl(Duration::from_secs(v));
            }
            Err(_) => panic!("Unable to parse MINECRAFT_CACHE_TTL environment variable"),
        };
    }

    for endpoint in HYPIXEL_ENDPOINTS {
        if let Ok(value) = env::var(format!("HYPIXEL_CACHE_TTL.{}", endpoint.0)) {
            match value.parse::<u64>() {
                Ok(ttl) => {
                    config = config.add_hypixel_cache_ttl(endpoint.1, Duration::from_secs(ttl));
                }
                Err(_) => {
                    panic!(
                        "Unable to parse HYPIXEL_CACHE_TTL.{} environment variable",
                        endpoint.0
                    )
                }
            }
        }
    }

    let web_data = Data::new(WebData {
        api: Mutex::new(RsPixel::from_config(api_key, config.into()).await?),
    });

    println!("Starting server...");
    HttpServer::new(move || {
        let mut app = App::new()
            .app_data(web_data.clone())
            .default_service(web::to(default));

        for endpoint in SERVER_ENDPOINTS {
            if let Ok(value) = env::var(format!("SERVER_ENDPOINT.{endpoint}")) {
                match value.parse::<bool>() {
                    Ok(enable) => {
                        if enable {
                            app = add_endpoint(app, endpoint);
                        }
                    }
                    Err(_) => {
                        panic!("Unable to parse SERVER_ENDPOINT.{endpoint} environment variable")
                    }
                }
            }
        }

        app
    })
    .bind((base_url, port))?
    .run()
    .await?;

    Ok(())
}
