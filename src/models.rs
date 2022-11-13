use serde::{Deserialize, Serialize};

use crate::submodels::{Bot, Bots, Server, Servers, Template, User, Users};

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseResponse {
    pub error: bool,
    pub status: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebsiteStatistics {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub servers: Servers,
    pub bots: Bots,
    pub users: Users,
    pub templates: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebsiteHealth {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub redis_ok: bool,
    pub mongo_ok: bool,
    pub redis_ping: u64,
    pub mongo_ping: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotInformation {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub bot: Bot,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotStatistics {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(rename = "guildCount")]
    pub guild_count: u64,
    #[serde(rename = "shardCount")]
    pub shard_count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInformation {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub server: Server,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateInformation {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub template: Template,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInformation {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub user: User,
}
