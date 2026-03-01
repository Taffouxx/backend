use iso8601_timestamp::Timestamp;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Serialize, Deserialize};

use super::File;

#[cfg(feature = "validator")]
use validator::Validate;

pub static RE_USERNAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\p{L}|[\d_.-])+$").unwrap());
pub static RE_DISPLAY_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[^\u200B\n\r]+$").unwrap());

// ВСЁ засовываем в auto_derived!, чтобы Револт сам повесил нужные трейты (Eq, PartialEq, JsonSchema)
auto_derived!(
    pub struct User {
        #[serde(rename = "_id")]
        pub id: String,
        pub username: String,
        pub discriminator: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub avatar: Option<File>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub relations: Vec<Relationship>,
        pub badges: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<UserStatus>,
        
        // Твои трофеи теперь внутри правильной структуры
        #[serde(default)]
        pub trophies: Vec<Trophy>,

        pub flags: u32,
        pub privileged: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bot: Option<BotInformation>,
        pub relationship: RelationshipStatus,
        pub online: bool,
    }

    pub enum FieldsUser {
        Avatar, StatusText, StatusPresence, StatusEmoji, ProfileContent, ProfileBackground, DisplayName, Internal,
    }

    #[derive(Default)]
    pub enum RelationshipStatus {
        #[default] None, User, Friend, Outgoing, Incoming, Blocked, BlockedOther,
    }

    pub struct Relationship {
        #[serde(rename = "_id")]
        pub user_id: String,
        pub status: RelationshipStatus,
    }

    pub enum Presence { Online, Idle, Focus, Busy, Invisible }

    #[derive(Default)]
    pub struct UserStatus {
        pub text: Option<String>,
        pub presence: Option<Presence>,
        pub emoji: Option<String>,
    }

    // Твоя структура трофея (в единственном экземпляре)
    #[derive(Default)]
    pub struct Trophy {
        pub id: String,
        pub title: String,
        pub description: Option<String>,
        pub icon: Option<String>,
        pub date: Option<String>,
    }

    pub struct BotInformation {
        #[serde(rename = "owner")]
        pub owner_id: String,
    }
);

// А вот блок конвертации всегда живет отдельно
impl From<crate::User> for User {
    fn from(value: crate::User) -> Self {
        // Оставляем дебаг, чтобы убедиться, что база отдает данные
        if value.id == "01KHEWJGGMN8RA5AW2620DGMK6" {
            println!("!!! DEBUG: DB TROPHIES: {:?}", value.trophies);
        }

        Self {
            id: value.id,
            username: value.username,
            discriminator: value.discriminator,
            display_name: value.display_name,
            
            // Явные типы спасают компилятор от паники
            avatar: value.avatar.map(|f| File::from(f)),
            relations: value.relations.map(|r| r.into_iter().map(|i| Relationship::from(i)).collect()).unwrap_or_default(),
            badges: value.badges.unwrap_or_default() as u32,
            status: value.status.map(|s| s.into(true)).flatten(),
            flags: value.flags.unwrap_or_default() as u32,
            privileged: value.privileged,
            bot: value.bot.map(|b| BotInformation::from(b)),
            relationship: RelationshipStatus::None,
            online: false,
            
            // Маппим трофеи
            trophies: value.trophies.unwrap_or_default(),
        }
    }
}