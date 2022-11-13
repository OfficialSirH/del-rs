use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Servers {
    pub total: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bots {
    pub total: u64,
    pub approved: u64,
    pub premium: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Users {
    pub total: u64,
    pub premium: u64,
    pub staff: Staff,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Staff {
    pub total: u64,
    pub mods: u64,
    pub assistants: u64,
    pub admins: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bot {
    pub id: String,
    pub name: String,
    pub prefix: String,
    pub tags: Vec<String>,
    #[serde(rename = "vanityUrl")]
    pub vanity_url: String,
    #[serde(rename = "serverCount")]
    pub server_count: u64,
    #[serde(rename = "shardCount")]
    pub shard_count: u64,
    pub flags: u64,
    #[serde(rename = "shortDesc")]
    pub short_desc: String,
    #[serde(rename = "longDesc")]
    pub long_desc: String,
    pub editors: Vec<String>,
    pub owner: Owner,
    pub avatar: Avatar,
    pub links: BotLinks,
    pub social: Social,
    pub status: BotStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Avatar {
    pub hash: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotLinks {
    pub invite: String,
    pub support: String,
    pub website: String,
    pub donation: String,
    pub repo: String,
    #[serde(rename = "privacyPolicy")]
    pub privacy_policy: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Social {
    pub twitter: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotStatus {
    pub approved: bool,
    #[serde(rename = "siteBot")]
    pub site_bot: bool,
    pub archived: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIPostStats {
    #[serde(rename = "guildCount")]
    pub guild_count: u64,
    #[serde(rename = "shardCount")]
    pub shard_count: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub id: String,
    pub name: String,
    #[serde(rename = "shortDesc")]
    pub short_desc: String,
    #[serde(rename = "longDesc")]
    pub long_desc: String,
    pub tags: Vec<String>,
    #[serde(rename = "previewChannel")]
    pub preview_channel: String,
    pub owner: Owner,
    pub icon: Icon,
    pub links: ServerLinks,
    pub status: ServerStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Icon {
    pub hash: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerLinks {
    pub website: String,
    pub donation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerStatus {
    #[serde(rename = "reviewRequired")]
    pub review_required: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub region: String,
    pub locale: String,
    #[serde(rename = "afkTimeout")]
    pub afk_timeout: u64,
    #[serde(rename = "verificationLevel")]
    pub verification_level: u64,
    #[serde(rename = "defaultMessageNotifications")]
    pub default_message_notifications: u64,
    #[serde(rename = "explicitContent")]
    pub explicit_content: u64,
    pub roles: Vec<TemplateRole>,
    pub channels: Vec<TemplateChannel>,
    #[serde(rename = "usageCount")]
    pub usage_count: u64,
    #[serde(rename = "shortDesc")]
    pub short_desc: String,
    #[serde(rename = "longDesc")]
    pub long_desc: String,
    pub tags: Vec<String>,
    #[serde(rename = "fromGuild")]
    pub from_guild: String,
    pub owner: Owner,
    pub icon: Icon,
    pub links: TemplateLinks,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateRole {
    pub name: String,
    pub color: u64,
    pub hoist: bool,
    pub position: u64,
    pub permissions: u64,
    pub managed: bool,
    pub mentionable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateChannel {
    #[serde(rename = "type")]
    pub type_: u64,
    pub name: String,
    pub nsfw: bool,
    pub permissions_overwrites: Option<Vec<PermissionOverwrite>>,
    pub rate_limit_per_user: u64,
    pub last_message_id: String,
    pub last_pin_timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionOverwrite {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: u64,
    pub allow: u64,
    pub deny: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateLinks {
    pub template: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub discrim: String,
    #[serde(rename = "fullUsername")]
    pub full_username: String,
    pub flags: u64,
    pub avatar: Avatar,
    pub profile: Profile,
    pub game: Game,
    pub rank: Rank,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub bio: String,
    pub links: UserLinks,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLinks {
    website: String,
    github: String,
    gitlab: String,
    twitter: String,
    instagram: String,
    snapchat: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub snakes: Snakes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snakes {
    #[serde(rename = "maxScore")]
    pub max_score: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rank {
    pub admin: bool,
    pub assistant: bool,
    #[serde(rename = "mod")]
    pub mod_: bool,
    pub tester: bool,
    pub translator: bool,
    pub covid: bool,
}
