use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
    Method,
};
use serde::{Deserialize, Serialize};

use crate::{
    errors::DelError,
    models::{
        BotInformation, BotStatistics, ServerInformation, TemplateInformation, UserInformation,
        WebsiteHealth, WebsiteStatistics,
    },
    submodels::APIPostStats,
};

#[derive(Debug)]
pub struct Del {
    pub token: String,
    pub id: String,
}

impl Default for Del {
    fn default() -> Self {
        Self {
            token: String::new(),
            id: String::new(),
        }
    }
}

impl Del {
    pub fn new(token: String, id: String) -> Self {
        Self { token, id }
    }

    async fn request<T>(
        &self,
        auth: Option<String>,
        method: Method,
        route: &str,
        body: Option<String>,
    ) -> Result<T, DelError>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            "User-Agent",
            format!(
                "del-rs ( {}, {} )",
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_REPOSITORY")
            )
            .parse()?,
        );
        if let Some(auth) = auth {
            headers.insert("Authorization", auth.parse::<HeaderValue>()?);
            headers.insert("Content-Type", "application/json".parse()?);
        }

        let res = client
            .request(
                method,
                format!("https://api.discordextremelist.xyz/v2{}", route),
            )
            .headers(headers)
            .body(body.unwrap_or("".to_owned()))
            .send()
            .await?;

        if res.status().is_success() {
            Ok(res.json::<T>().await?)
        } else {
            Err(res.json::<DelError>().await?)
        }
    }

    pub async fn get_website_stats(&self) -> Result<WebsiteStatistics, DelError> {
        let res = self
            .request::<WebsiteStatistics>(None, Method::GET, "/stats", None)
            .await?;

        Ok(res)
    }

    pub async fn get_website_health(&self) -> Result<WebsiteHealth, DelError> {
        let res = self
            .request::<WebsiteHealth>(None, Method::GET, "/health", None)
            .await?;

        Ok(res)
    }

    pub async fn get_bot_info(&self, id: String) -> Result<BotInformation, DelError> {
        let res = self
            .request::<BotInformation>(None, Method::GET, &format!("/bot/{}", id), None)
            .await?;

        Ok(res)
    }

    pub async fn post_stats(
        &self,
        guild_count: u64,
        shard_count: Option<u64>,
    ) -> Result<BotStatistics, DelError> {
        let res = self
            .request::<BotStatistics>(
                Some(self.token.clone()),
                Method::POST,
                &format!("/bot/{}/stats", self.id),
                Some(
                    serde_json::to_string(&APIPostStats {
                        guild_count,
                        shard_count,
                    })
                    .unwrap(),
                ),
            )
            .await?;

        Ok(res)
    }

    pub async fn get_server_info(&self, id: String) -> Result<ServerInformation, DelError> {
        let res = self
            .request::<ServerInformation>(None, Method::GET, &format!("/server/{}", id), None)
            .await?;

        Ok(res)
    }

    pub async fn get_template_info(&self, id: String) -> Result<TemplateInformation, DelError> {
        let res = self
            .request::<TemplateInformation>(None, Method::GET, &format!("/template/{}", id), None)
            .await?;

        Ok(res)
    }

    pub async fn get_user_info(&self, id: String) -> Result<UserInformation, DelError> {
        let res = self
            .request::<UserInformation>(None, Method::GET, &format!("/user/{}", id), None)
            .await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_website_stats() {
        let del = Del::default();
        let res = del.get_website_stats().await.unwrap();

        assert_eq!(res.base.error, false);
        assert_eq!(res.base.status, 200);
    }

    #[tokio::test]
    async fn test_get_website_health() {
        let del = Del::default();
        let res = del.get_website_health().await.unwrap();

        assert_eq!(res.base.error, false);
        assert_eq!(res.base.status, 200);
    }

    #[tokio::test]
    async fn test_get_bot_info() {
        let del = Del::default();
        let res = del
            .get_bot_info("568254611354419211".to_owned())
            .await
            .unwrap();

        assert_eq!(res.base.error, false);
        assert_eq!(res.base.status, 200);
    }

    #[tokio::test]
    async fn test_post_stats() {
        let del = Del::default();
        let res = del.post_stats(0, Some(0)).await;

        assert!(res.is_err());
        assert_eq!(
            res.expect_err("Somehow didn't return an error from an unauthorized request")
                .status,
            Some(403)
        );
    }

    #[tokio::test]
    async fn test_get_server_info() {
        let del = Del::default();
        let res = del
            .get_server_info("568567800910839811".to_owned())
            .await
            .unwrap();

        assert_eq!(res.base.error, false);
        assert_eq!(res.base.status, 200);
    }

    #[tokio::test]
    async fn test_get_template_info() {
        let del = Del::default();
        let res = del
            .get_template_info("4sgbPdCjzAYU".to_owned())
            .await
            .unwrap();

        assert_eq!(res.base.error, false);
        assert_eq!(res.base.status, 200);
    }

    #[tokio::test]
    async fn test_get_user_info() {
        let del = Del::default();
        let res = del
            .get_user_info("744603004493365330".to_owned())
            .await
            .unwrap();

        assert_eq!(res.base.error, false);
        assert_eq!(res.base.status, 200);
    }
}
