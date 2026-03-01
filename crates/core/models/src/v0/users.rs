use iso8601_timestamp::Timestamp;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Serialize, Deserialize};

use super::File;

#[cfg(feature = "validator")]
use validator::Validate;

pub static RE_USERNAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\p{L}|[\d_.-])+$").unwrap());
pub static RE_DISPLAY_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[^\u200B\n\r]+$").unwrap());

// Весь код сидит в макросе, чтобы генерировались схемы для API
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
        
        // ТВОИ КУБКИ
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

    // СТРУКТУРА КУБКА
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

    // СТРУКТУРЫ, КОТОРЫЕ БЫЛИ СТЕРТЫ (возвращаем на базу)
    #[derive(Default)]
    pub struct UserProfile {
        pub content: Option<String>,
        pub background: Option<File>,
    }

    #[derive(Default)]
    pub struct UserVoiceState {
        pub channel_id: String,
        pub session_id: String,
    }
);

// Блок конвертации данных из базы
impl From<crate::User> for User {
    fn from(value: crate::User) -> Self {
        // Дебаг для логов
        if value.id == "01KHEWJGGMN8RA5AW2620DGMK6" {
            println!("!!! DEBUG: DB TROPHIES: {:?}", value.trophies);
        }

        Self {
            id: value.id,
            username: value.username,
            discriminator: value.discriminator,
            display_name: value.display_name,
            
            // Явные конвертации, чтобы компилятор не задавал вопросов
            avatar: value.avatar.map(|f| File::from(f)),
            relations: value.relations.map(|r| r.into_iter().map(|i| Relationship::from(i)).collect()).unwrap_or_default(),
            badges: value.badges.unwrap_or_default() as u32,
            status: value.status.map(|s| s.into(true)).flatten(),
            flags: value.flags.unwrap_or_default() as u32,
            privileged: value.privileged,
            bot: value.bot.map(|b| BotInformation::from(b)),
            relationship: RelationshipStatus::None,
            online: false,
            
            // Закидываем кубки
            trophies: value.trophies.unwrap_or_default(),
        }
    }
}