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
        pub id: String,
        pub joined_at: String,
        pub is_receiving: bool,
        pub is_publishing: bool,
        pub screensharing: bool,
        pub camera: bool,
    }

    pub struct PartialUser {
        #[serde(rename = "_id")]
        pub id: Option<String>,
        pub username: Option<String>,
        pub discriminator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub avatar: Option<File>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub relations: Option<Vec<Relationship>>,
        pub badges: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<UserStatus>,
        #[serde(default)]
        pub trophies: Option<Vec<Trophy>>,
        pub flags: Option<u32>,
        pub privileged: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bot: Option<BotInformation>,
        pub relationship: Option<RelationshipStatus>,
        pub online: Option<bool>,
    }

    #[derive(Default)]
    pub struct PartialUserVoiceState {
        pub channel_id: Option<String>,
        pub session_id: Option<String>,
        pub id: Option<String>,
        pub joined_at: Option<String>,
        pub is_receiving: Option<bool>,
        pub is_publishing: Option<bool>,
        pub screensharing: Option<bool>,
        pub camera: Option<bool>,
    }

    );

pub type UserBadges = u32;
pub type UserFlags = u32;

/// Data for editing a user's status
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataEditStatus {
    /// Custom status text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Presence option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<Presence>,
}

/// Data for editing a user's profile
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataEditProfile {
    /// Profile text content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Background attachment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
}

/// Data for editing a user
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataEditUser {
    /// New display name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Avatar attachment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Status update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DataEditStatus>,
    /// Profile update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<DataEditProfile>,
    /// Bitfield of user badges (privileged only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badges: Option<i32>,
    /// Bitfield of user flags (privileged only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    /// Fields to remove from the user object
    #[serde(default)]
    pub remove: Vec<FieldsUser>,
}

/// Data for sending a friend request
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataSendFriendRequest {
    /// Username and discriminator combo, e.g. `User#1234`
    pub username: String,
}

/// Response containing mutual relationships between two users
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MutualResponse {
    pub users: Vec<String>,
    pub servers: Vec<String>,
    pub channels: Vec<String>,
}

/// Response containing a user's flags
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FlagResponse {
    pub flags: i32,
}
