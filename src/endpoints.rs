use crate::{
    structs::{
        AuctionQuery, GuildQuery, PlayerQuery, RecentGamesQuery, ResourcesPath,
        SkyblockAuctionQuery, SkyblockBingoQuery, SkyblockProfileQuery, SkyblockProfilesQuery,
        StatusQuery, WebData,
    },
    utils::{bad_request, error_response, ok, HYPIXEL_ENDPOINTS},
};
use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    get,
    web::{resource, Data, Path, Query, Redirect},
    App, Responder,
};

pub fn add_endpoint<T>(app: App<T>, value: &str) -> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), InitError = (), Error = actix_web::Error>,
{
    match value {
        "KEY" => app.service(key),
        "BOOSTERS" => app.service(boosters),
        "LEADERBOARDS" => app.service(leaderboards),
        "PUNISHMENT_STATS" => app.service(punishment_stats),
        "PLAYER" => app.service(player),
        "GUILD" => app.service(guild),
        "COUNTS" => app.service(counts),
        "STATUS" => app.service(status),
        "RECENT_GAMES" => app.service(recent_games),
        "SKYBLOCK_PROFILES" => app.service(skyblock_profiles),
        "SKYBLOCK_PROFILE" => app.service(skyblock_profile),
        "SKYBLOCK_BINGO" => app.service(skyblock_bingo),
        "SKYBLOCK_NEWS" => app.service(skyblock_auction),
        "SKYBLOCK_AUCTION" => app.service(skyblock_auction),
        "SKYBLOCK_AUCTIONS" => app.service(skyblock_auctions),
        "SKYBLOCK_AUCTIONS_ENDED" => app.service(skyblock_auctions_ended),
        "SKYBLOCK_BAZAAR" => app.service(skyblock_bazaar),
        "SKYBLOCK_FIRESALES" => app.service(skyblock_fire_sales),
        "RESOURCES" => app.service(
            resource([
                "resources",
                "resources/{resource}",
                "resources/{resource}/{sub_resource}",
            ])
            .to(resources),
        ),
        _ => panic!("Unable to parse server endpoint from {value}"),
    }
}

pub async fn default() -> impl Responder {
    Redirect::to("https://github.com/kr45732/rs-pixel-backend").permanent()
}

#[get("/key")]
async fn key(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_key().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/boosters")]
async fn boosters(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_boosters().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/leaderboards")]
async fn leaderboards(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_leaderboards().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/punishmentstats")]
async fn punishment_stats(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_punishment_stats().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/player")]
async fn player(web_data: Data<WebData>, query: Query<PlayerQuery>) -> impl Responder {
    if query.username.is_none() && query.uuid.is_none() {
        return bad_request("Missing one or more fields [username, uuid]");
    }

    let uuid;
    if let Some(uuid_unwrap) = &query.uuid {
        uuid = uuid_unwrap.to_string();
    } else {
        let username = query.username.clone().unwrap();
        match web_data.api.lock().await.username_to_uuid(&username).await {
            Ok(res) => uuid = res.uuid,
            Err(err) => return error_response(err),
        }
    }

    match web_data.api.lock().await.get_player(&uuid).await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/guild")]
async fn guild(web_data: Data<WebData>, query: Query<GuildQuery>) -> impl Responder {
    let res;
    if let Some(id) = &query.id {
        res = web_data.api.lock().await.get_guild_by_id(id).await
    } else if let Some(name) = &query.name {
        res = web_data.api.lock().await.get_guild_by_name(name).await
    } else if query.player.is_some() || query.username.is_some() {
        let uuid;
        if let Some(uuid_unwrap) = &query.player {
            uuid = uuid_unwrap.to_string();
        } else {
            let username = query.username.clone().unwrap();
            match web_data.api.lock().await.username_to_uuid(&username).await {
                Ok(res) => uuid = res.uuid,
                Err(err) => return error_response(err),
            }
        }

        res = web_data.api.lock().await.get_guild_by_player(&uuid).await
    } else {
        return bad_request("Missing one or more fields [id, name, player, username]");
    }

    match res {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/counts")]
async fn counts(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_counts().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/status")]
async fn status(web_data: Data<WebData>, query: Query<StatusQuery>) -> impl Responder {
    if query.username.is_none() && query.uuid.is_none() {
        return bad_request("Missing one or more fields [username, uuid]");
    }

    let uuid;
    if let Some(uuid_unwrap) = &query.uuid {
        uuid = uuid_unwrap.to_string();
    } else {
        let username = query.username.clone().unwrap();
        match web_data.api.lock().await.username_to_uuid(&username).await {
            Ok(res) => uuid = res.uuid,
            Err(err) => return error_response(err),
        }
    }

    match web_data.api.lock().await.get_status(&uuid).await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/recentGames")]
async fn recent_games(web_data: Data<WebData>, query: Query<RecentGamesQuery>) -> impl Responder {
    if query.username.is_none() && query.uuid.is_none() {
        return bad_request("Missing one or more fields [username, uuid]");
    }

    let uuid;
    if let Some(uuid_unwrap) = &query.uuid {
        uuid = uuid_unwrap.to_string();
    } else {
        let username = query.username.clone().unwrap();
        match web_data.api.lock().await.username_to_uuid(&username).await {
            Ok(res) => uuid = res.uuid,
            Err(err) => return error_response(err),
        }
    }

    match web_data.api.lock().await.get_recent_games(&uuid).await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/skyblock/profiles")]
async fn skyblock_profiles(
    web_data: Data<WebData>,
    query: Query<SkyblockProfilesQuery>,
) -> impl Responder {
    if query.username.is_none() && query.uuid.is_none() {
        return bad_request("Missing one or more fields [username, uuid]");
    }

    let uuid;
    if let Some(uuid_unwrap) = &query.uuid {
        uuid = uuid_unwrap.to_string();
    } else {
        let username = query.username.clone().unwrap();
        match web_data.api.lock().await.username_to_uuid(&username).await {
            Ok(res) => uuid = res.uuid,
            Err(err) => return error_response(err),
        }
    }

    match web_data.api.lock().await.get_skyblock_profiles(&uuid).await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/skyblock/profile")]
async fn skyblock_profile(
    web_data: Data<WebData>,
    query: Query<SkyblockProfileQuery>,
) -> impl Responder {
    if let Some(profile) = &query.profile {
        match web_data
            .api
            .lock()
            .await
            .get_skyblock_profile(profile)
            .await
        {
            Ok(res) => ok(res),
            Err(err) => error_response(err),
        }
    } else {
        bad_request("Missing one or more fields [profile]")
    }
}

#[get("/skyblock/bingo")]
async fn skyblock_bingo(
    query: Query<SkyblockBingoQuery>,
    web_data: Data<WebData>,
) -> impl Responder {
    if query.username.is_none() && query.uuid.is_none() {
        return bad_request("Missing one or more fields [username, uuid]");
    }

    let uuid;
    if let Some(uuid_unwrap) = &query.uuid {
        uuid = uuid_unwrap.to_string();
    } else {
        let username = query.username.clone().unwrap();
        match web_data.api.lock().await.username_to_uuid(&username).await {
            Ok(res) => uuid = res.uuid,
            Err(err) => return error_response(err),
        }
    }

    match web_data.api.lock().await.get_skyblock_bingo(&uuid).await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/skyblock/news")]
async fn skyblock_news(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_skyblock_news().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/skyblock/auction")]
async fn skyblock_auction(web_data: Data<WebData>, query: Query<AuctionQuery>) -> impl Responder {
    let res;
    if query.player.is_some() || query.username.is_some() {
        let uuid;
        if let Some(uuid_unwrap) = &query.player {
            uuid = uuid_unwrap.to_string();
        } else {
            let username = query.username.clone().unwrap();
            match web_data.api.lock().await.username_to_uuid(&username).await {
                Ok(res) => uuid = res.uuid,
                Err(err) => return error_response(err),
            }
        }

        res = web_data
            .api
            .lock()
            .await
            .get_skyblock_auction_by_player(&uuid)
            .await
    } else if let Some(uuid) = &query.uuid {
        res = web_data
            .api
            .lock()
            .await
            .get_skyblock_auction_by_uuid(uuid)
            .await
    } else if let Some(profile) = &query.profile {
        res = web_data
            .api
            .lock()
            .await
            .get_skyblock_auction_by_profile(profile)
            .await
    } else {
        return bad_request("Missing one or more fields [player, uuid, profile, username]");
    }

    match res {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/skyblock/auctions")]
async fn skyblock_auctions(
    web_data: Data<WebData>,
    query: Query<SkyblockAuctionQuery>,
) -> impl Responder {
    match web_data
        .api
        .lock()
        .await
        .get_skyblock_auctions(query.page.unwrap_or(0))
        .await
    {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/skyblock/auctions_ended")]
async fn skyblock_auctions_ended(web_data: Data<WebData>) -> impl Responder {
    match web_data
        .api
        .lock()
        .await
        .get_skyblock_auctions_ended()
        .await
    {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/skyblock/bazaar")]
async fn skyblock_bazaar(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_skyblock_bazaar().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/skyblock/firesales")]
async fn skyblock_fire_sales(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_skyblock_fire_sales().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

async fn resources(web_data: Data<WebData>, path: Path<ResourcesPath>) -> impl Responder {
    if let Some(resource) = &path.resource {
        let resource_path = if let Some(sub_resource) = &path.sub_resource {
            format!("resources/{resource}/{sub_resource}")
        } else {
            format!("resources/{resource}")
        };

        for endpoint in HYPIXEL_ENDPOINTS {
            if endpoint.2 && endpoint.1.get_path() == resource_path {
                return match web_data.api.lock().await.get_resources(endpoint.1).await {
                    Ok(res) => ok(res),
                    Err(err) => error_response(err),
                };
            }
        }

        bad_request("Unknown resource provided")
    } else {
        bad_request("No resource provided")
    }
}
