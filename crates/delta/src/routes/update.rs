use lru::LruCache;
use once_cell::sync::Lazy;
use rocket::serde::json::Json;
use rocket::{get, Route};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::sync::Mutex;
use revolt_rocket_okapi::revolt_okapi::openapi3::OpenApi;

const DESKTOP_REPO: &str = "Taffouxx/zeelo-desktop";
const CACHE_TTL_SECONDS: u64 = 900;
const GITHUB_API_URL: &str = "https://api.github.com";

static CACHE: Lazy<Mutex<LruCache<String, CachedRelease>>> = 
    Lazy::new(|| Mutex::new(LruCache::new(2)));

fn get_extension_for_target(target: &str) -> &'static str {
    if target.starts_with("windows") {
        "msi"
    } else if target.starts_with("macos") {
        "dmg"
    } else {
        "AppImage"
    }
}

fn get_github_token() -> Option<String> {
    std::env::var("GITHUB_TOKEN").ok().filter(|s| !s.is_empty())
}

async fn fetch_latest_release() -> Result<GitHubRelease, String> {
    let url = format!("{}/repos/{}/releases/latest", GITHUB_API_URL, DESKTOP_REPO);
    
    let mut request = reqwest::Client::new()
        .get(&url)
        .header("User-Agent", "Zeelo-Backend")
        .header("Accept", "application/vnd.github+json");
    
    if let Some(token) = get_github_token() {
        request = request.header("Authorization", format!("Bearer {}", token));
    }
    
    let response = request.send().await
        .map_err(|e| format!("Failed to fetch release: {}", e))?;
    
    if response.status() == 404 {
        return Err("No releases found".to_string());
    }
    
    if !response.status().is_success() {
        return Err(format!("GitHub API error: {}", response.status()));
    }
    
    response.json::<GitHubRelease>().await
        .map_err(|e| format!("Failed to parse release: {}", e))
}

fn version_from_tag(tag: &str) -> String {
    tag.trim_start_matches('v').to_string()
}

fn find_signature_asset(release: &GitHubRelease) -> Option<&GitHubAsset> {
    release.assets.iter().find(|asset| {
        let name_lower = asset.name.to_lowercase();
        name_lower.ends_with(".sig")
    })
}

async fn fetch_signature(asset: &GitHubAsset) -> Result<String, String> {
    let mut request = reqwest::Client::new()
        .get(&asset.browser_download_url)
        .header("User-Agent", "Zeelo-Backend");
    
    if let Some(token) = get_github_token() {
        request = request.header("Authorization", format!("Bearer {}", token));
    }
    
    let response = request.send().await
        .map_err(|e| format!("Failed to fetch signature: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to download signature: {}", response.status()));
    }
    
    response.text().await
        .map(|s| s.trim().to_string())
        .map_err(|e| format!("Failed to read signature: {}", e))
}

async fn get_cached_release() -> Result<CachedRelease, String> {
    let cache_key = "latest".to_string();
    
    if let Ok(mut cache) = CACHE.lock() {
        if let Some(cached) = cache.get(&cache_key) {
            if !cached.is_expired() {
                return Ok(cached.clone());
            }
        }
    }
    
    let release = fetch_latest_release().await?;
    let version = version_from_tag(&release.tag_name);
    
    let target_sig = if let Some(sig_asset) = find_signature_asset(&release) {
        fetch_signature(sig_asset).await.ok()
    } else {
        None
    };
    
    let cached = CachedRelease {
        version,
        tag_name: release.tag_name.clone(),
        body: release.body.clone().unwrap_or_default(),
        published_at: release.published_at.clone(),
        platforms: release.assets.iter().filter_map(|a| {
            let ext = if a.name.to_lowercase().contains("windows") {
                "msi"
            } else if a.name.to_lowercase().contains("macos") {
                "dmg"
            } else {
                "AppImage"
            };
            if a.name.to_lowercase().contains("x86_64") || a.name.to_lowercase().contains("x64") {
                Some((format!("{}-x86_64", get_platform_prefix(&a.name)), (
                    a.browser_download_url.clone(),
                    target_sig.clone().unwrap_or_default()
                )))
            } else if a.name.to_lowercase().contains("aarch64") || a.name.to_lowercase().contains("arm64") {
                Some((format!("{}-aarch64", get_platform_prefix(&a.name)), (
                    a.browser_download_url.clone(),
                    target_sig.clone().unwrap_or_default()
                )))
            } else {
                None
            }
        }).collect(),
        cached_at: std::time::Instant::now(),
    };
    
    if let Ok(mut cache) = CACHE.lock() {
        cache.put(cache_key, cached.clone());
    }
    
    Ok(cached)
}

fn get_platform_prefix(asset_name: &str) -> &'static str {
    let name_lower = asset_name.to_lowercase();
    if name_lower.contains("windows") {
        "windows"
    } else if name_lower.contains("macos") {
        "macos"
    } else if name_lower.contains("linux") {
        "linux"
    } else {
        "unknown"
    }
}

#[derive(Debug, Clone)]
struct CachedRelease {
    version: String,
    tag_name: String,
    body: String,
    published_at: String,
    platforms: std::collections::HashMap<String, (String, String)>,
    cached_at: std::time::Instant,
}

impl CachedRelease {
    fn is_expired(&self) -> bool {
        self.cached_at.elapsed().as_secs() > CACHE_TTL_SECONDS
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GitHubRelease {
    tag_name: String,
    name: Option<String>,
    body: Option<String>,
    published_at: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

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

fn parse_date(date_str: &str) -> String {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        dt.format("%Y-%m-%d").to_string()
    } else {
        date_str.split('T').next().unwrap_or(date_str).to_string()
    }
}

fn normalize_target(target: &str) -> String {
    target.replace("_", "-").to_lowercase()
}

fn matches_target(platform_key: &str, target: &str) -> bool {
    let target_norm = normalize_target(target);
    
    let expected_prefix = if target.contains("windows") {
        "windows"
    } else if target.contains("macos") {
        "macos"
    } else if target.contains("linux") {
        "linux"
    } else {
        return false;
    };
    
    let expected_arch = if target.contains("aarch64") || target.contains("arm64") {
        "aarch64"
    } else {
        "x86_64"
    };
    
    platform_key.to_lowercase() == format!("{}-{}", expected_prefix, expected_arch)
}

#[openapi(tag = "Auto-Update")]
#[get("/<target>/<current_version>")]
pub async fn get_update(
    target: &str,
    current_version: &str,
) -> Option<Json<UpdateInfo>> {
    let normalized_target = normalize_target(target);
    let _ = normalized_target; // used for debug logging if needed
    
    let valid_targets = [
        "windows-x86_64", "windows-aarch64",
        "linux-x86_64", "linux-aarch64",
        "macos-x86_64", "macos-aarch64"
    ];
    
    if !valid_targets.contains(&normalized_target.as_str()) {
        return None;
    }
    
    let release = match get_cached_release().await {
        Ok(r) => r,
        Err(e) => {
            log::warn!("Failed to fetch release: {}", e);
            return None;
        }
    };
    
    let current_ver = current_version.trim_start_matches('v');
    let latest_ver = release.version.trim_start_matches('v');
    
    if current_ver == latest_ver {
        return None;
    }
    
    let mut platforms = std::collections::HashMap::new();
    
    for (platform_key, (url, sig)) in &release.platforms {
        if matches_target(platform_key, target) {
            platforms.insert(
                normalized_target.clone(),
                PlatformInfo {
                    url: url.clone(),
                    signature: sig.clone(),
                },
            );
            break;
        }
    }
    
    if platforms.is_empty() {
        return None;
    }
    
    Some(Json(UpdateInfo {
        version: release.version.clone(),
        date: parse_date(&release.published_at),
        body: if release.body.is_empty() {
            format!("New version {} is available! You have {}.", latest_ver, current_ver)
        } else {
            release.body.clone()
        },
        platforms,
    }))
}

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![get_update]
}
