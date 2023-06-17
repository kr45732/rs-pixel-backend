use crate::{
    structs::{
        AuctionQuery, GuildQuery, PlayerQuery, RecentGamesQuery, ResourcesPath,
        SkyblockAuctionQuery, SkyblockBingoQuery, SkyblockProfileQuery, SkyblockProfilesQuery,
        StatusQuery, WebData,
    },
    utils::{bad_request, error_response, ok, RESOURCES},
};
use actix_web::{
    get,
    web::{Data, Path, Query},
    Responder,
};

#[get("/key")]
pub async fn key(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_key().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/boosters")]
pub async fn boosters(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_boosters().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/leaderboards")]
pub async fn leaderboards(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_leaderboards().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/punishmentstats")]
pub async fn punishment_stats(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_punishment_stats().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/player")]
pub async fn player(web_data: Data<WebData>, query: Query<PlayerQuery>) -> impl Responder {
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
pub async fn guild(web_data: Data<WebData>, query: Query<GuildQuery>) -> impl Responder {
    let res;
    if let Some(id) = &query.id {
        res = web_data.api.lock().await.get_guild_by_id(id).await
    } else if let Some(name) = &query.name {
        res = web_data.api.lock().await.get_guild_by_name(name).await
    } else if query.player.is_some() || query.player_username.is_some() {
        let uuid;
        if let Some(uuid_unwrap) = &query.player {
            uuid = uuid_unwrap.to_string();
        } else {
            let username = query.player_username.clone().unwrap();
            match web_data.api.lock().await.username_to_uuid(&username).await {
                Ok(res) => uuid = res.uuid,
                Err(err) => return error_response(err),
            }
        }

        res = web_data.api.lock().await.get_guild_by_player(&uuid).await
    } else {
        return bad_request("Missing one or more fields [id, name, player, player_username]");
    }

    match res {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/counts")]
pub async fn counts(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_counts().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/status")]
pub async fn status(web_data: Data<WebData>, query: Query<StatusQuery>) -> impl Responder {
    if let Some(uuid) = &query.uuid {
        match web_data.api.lock().await.get_status(uuid).await {
            Ok(res) => ok(res),
            Err(err) => error_response(err),
        }
    } else {
        bad_request("Missing one or more fields [uuid]")
    }
}

#[get("/recentGames")]
pub async fn recent_games(
    web_data: Data<WebData>,
    query: Query<RecentGamesQuery>,
) -> impl Responder {
    if let Some(uuid) = &query.uuid {
        match web_data.api.lock().await.get_recent_games(uuid).await {
            Ok(res) => ok(res),
            Err(err) => error_response(err),
        }
    } else {
        bad_request("Missing one or more fields [uuid]")
    }
}

#[get("/skyblock/profiles")]
pub async fn skyblock_profiles(
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
pub async fn skyblock_profile(
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
pub async fn skyblock_bingo(
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
pub async fn skyblock_news(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_skyblock_news().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/skyblock/auction")]
pub async fn skyblock_auction(
    web_data: Data<WebData>,
    query: Query<AuctionQuery>,
) -> impl Responder {
    let res;
    if query.player.is_some() || query.player_username.is_some() {
        let uuid;
        if let Some(uuid_unwrap) = &query.player {
            uuid = uuid_unwrap.to_string();
        } else {
            let username = query.player_username.clone().unwrap();
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
        return bad_request("Missing one or more fields [player, uuid, profile, player_username]");
    }

    match res {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/skyblock/auctions")]
pub async fn skyblock_auctions(
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
pub async fn skyblock_auctions_ended(web_data: Data<WebData>) -> impl Responder {
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
pub async fn skyblock_bazaar(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_skyblock_bazaar().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

#[get("/skyblock/firesales")]
pub async fn skyblock_fire_sales(web_data: Data<WebData>) -> impl Responder {
    match web_data.api.lock().await.get_skyblock_fire_sales().await {
        Ok(res) => ok(res),
        Err(err) => error_response(err),
    }
}

pub async fn resources(web_data: Data<WebData>, path: Path<ResourcesPath>) -> impl Responder {
    if let Some(resource) = &path.resource {
        let mut resource_path = format!("resources/{resource}");

        if let Some(sub_resource) = &path.sub_resource {
            resource_path = format!("resources/{resource}/{sub_resource}");
        }

        for resource_enum in RESOURCES {
            if resource_enum.get_path() == resource_path {
                return match web_data.api.lock().await.get_resources(resource_enum).await {
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
