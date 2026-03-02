use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Serialize, Deserialize};

use super::File;

#[cfg(feature = "validator")]
use validator::Validate;

pub static RE_USERNAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\p{L}|[\d_.-])+$").unwrap());
pub static RE_DISPLAY_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[^\u200B\n\r]+$").unwrap());

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
        
        // КУБКИ
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

    #[derive(Default)]
    pub struct UserProfile {
        pub content: Option<String>,
        pub background: Option<File>,
    }

    pub struct UserVoiceState {
        pub channel_id: String,
        pub session_id: String,
    }
);

