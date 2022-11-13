# del-rs API Wrapper

This is a Rust API wrapper for Discord Extreme List's API.

## Getting Started

#### using the API Wrapper

```rs
use del_rs::del::Del;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // posting stats - requires Del to be built with token and id to work
  let del = Del::new("DEL TOKEN", "BOT ID");

  del.post_stats(1000, None).await; // Result<BotStatistics, DelError>

  // methods that don't require authorization
  let del = Del::default();

  del.get_website_stats().await; // Result<WebsiteStatistics, DelError>

  del.get_website_health().await; // Result<WebsiteHealth, DelError>

  del.get_bot_info("BOT ID").await; // Result<BotInformation, DelError>
  
  del.get_server_info("SERVER ID").await; // Result<ServerInformation, DelError>

  del.get_template_info("TEMPLATE ID").await; // Result<TemplateInformation, DelError>

  del.get_user_info("USER ID").await; // Result<UserInformation, DelError>
}
```
