use rocket::serde::json::Json;
use rocket::{get, Route};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use revolt_rocket_okapi::revolt_okapi::openapi3::OpenApi;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct PlatformInfo {
    pub url: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct UpdateInfo {
    pub version: String,
    pub date: String,
    pub body: String,
    pub platforms: std::collections::HashMap<String, PlatformInfo>,
}

#[openapi(tag = "Auto-Update")]
#[get("/<target>/<current_version>")]
pub async fn get_update(
    target: &str,
    current_version: &str,
) -> Option<Json<UpdateInfo>> {
    if target != "windows-x86_64" 
        && target != "windows-aarch64" 
        && target != "linux-x86_64" 
        && target != "linux-aarch64" 
        && target != "macos-x86_64" 
        && target != "macos-aarch64" 
    {
        return None;
    }

    let latest_version = env!("CARGO_PKG_VERSION");

    if current_version == latest_version {
        return None;
    }

    let mut platforms = std::collections::HashMap::new();
    platforms.insert(
        target.to_string(),
        PlatformInfo {
            url: format!(
                "https://releases.zeelo.chat/zeelo-desktop/{}/zeelo-desktop-{}.{}",
                latest_version,
                latest_version,
                if target.starts_with("windows") { "msi" } else { "AppImage" }
            ),
            signature: "".to_string(),
        },
    );

    Some(Json(UpdateInfo {
        version: latest_version.to_string(),
        date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        body: format!(
            "New version {} is available! You have {}.",
            latest_version, current_version
        ),
        platforms,
    }))
}

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![get_update]
}
