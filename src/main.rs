mod endpoints;
mod structs;
mod utils;

use crate::{
    endpoints::{add_endpoint, default},
    structs::WebData,
    utils::{HYPIXEL_ENDPOINTS, SERVER_ENDPOINTS},
};
use actix_governor::{Governor, GovernorConfigBuilder, KeyExtractor, SimpleKeyExtractionError};
use actix_web::{
    dev::ServiceRequest,
    web::{self, Data},
    App, HttpServer,
};
use dotenv::dotenv;
use rs_pixel::{util::minecraft::ApiType, ConfigBuilder, RateLimitStrategy, RsPixel};
use std::{collections::HashMap, env, error::Error, sync::Mutex, time::Duration};

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

    let mut governor_conf = GovernorConfigBuilder::default();
    let mut use_governor = false;
    if let Ok(value) = env::var("SERVER.PERIOD") {
        match value.parse::<u64>() {
            Ok(period) => {
                if period == 0 {
                    panic!("Unable to parse SERVER.PERIOD environment variable");
                } else {
                    governor_conf.per_millisecond(period);
                    use_governor = true;
                }
            }
            Err(_) => {
                panic!("Unable to parse SERVER.PERIOD environment variable");
            }
        }
    }
    if let Ok(value) = env::var("SERVER.BURST") {
        match value.parse::<u32>() {
            Ok(burst) => {
                if burst == 0 {
                    panic!("Unable to parse SERVER.PERIOD environment variable");
                } else {
                    governor_conf.burst_size(burst);
                    use_governor = true;
                }
            }
            Err(_) => {
                panic!("Unable to parse SERVER.BURST environment variable");
            }
        }
    }
    let governor = governor_conf
        .use_headers()
        .permissive(!use_governor)
        .key_extractor(CachingKeyExtractor)
        .finish()
        .unwrap();

    println!("Starting server...");
    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Governor::new(&governor))
            .app_data(web_data.clone())
            .default_service(web::to(default));

        for endpoint in SERVER_ENDPOINTS {
            if let Ok(value) = env::var(format!("SERVER.ENDPOINT.{endpoint}")) {
                match value.parse::<bool>() {
                    Ok(enable) => {
                        if enable {
                            app = app.configure(|conf| add_endpoint(conf, endpoint));
                        }
                    }
                    Err(_) => {
                        panic!("Unable to parse SERVER.ENDPOINT.{endpoint} environment variable")
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

#[derive(Clone)]
struct CachingKeyExtractor;
impl KeyExtractor for CachingKeyExtractor {
    type Key = String;
    type KeyExtractionError = SimpleKeyExtractionError<&'static str>;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        if req
            .app_data::<Data<WebData>>()
            .unwrap()
            .api
            .lock()
            .unwrap()
            .is_cached(
                &req.path()[1..],
                web::Query::<HashMap<String, String>>::from_query(req.query_string())
                    .unwrap()
                    .into_inner(),
            )
        {
            return Ok(String::new());
        }

        req.peer_addr()
            .map(|socket| socket.ip().to_string())
            .ok_or_else(|| {
                SimpleKeyExtractionError::new("Could not extract peer IP address from request")
            })
    }

    fn whitelisted_keys(&self) -> Vec<Self::Key> {
        vec![String::new()]
    }
}
