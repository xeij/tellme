// auto_update.rs - Auto-update functionality
// This module checks GitHub for new releases and notifies users

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use semver::Version;
use std::time::Duration;

const GITHUB_REPO: &str = "xeij/tellme"; // Replace with actual repo
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const UPDATE_CHECK_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Deserialize, Serialize)]
struct GitHubRelease {
    tag_name: String,
    name: String,
    html_url: String,
    body: String,
    draft: bool,
    prerelease: bool,
}

pub struct UpdateChecker {
    client: Client,
}

impl UpdateChecker {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(UPDATE_CHECK_TIMEOUT)
                .user_agent("tellme/0.2.0")
                .build()
                .unwrap_or_default(),
        }
    }

    /// Check for updates from GitHub releases
    pub async fn check_for_updates(&self) -> Result<Option<UpdateInfo>> {
        let url = format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            // Silently fail if we can't check for updates
            return Ok(None);
        }

        let release: GitHubRelease = response.json().await?;
        
        // Skip drafts and prereleases
        if release.draft || release.prerelease {
            return Ok(None);
        }

        // Parse versions
        let current_version = Version::parse(CURRENT_VERSION)?;
        let latest_version = Version::parse(&release.tag_name.trim_start_matches('v'))?;

        if latest_version > current_version {
            Ok(Some(UpdateInfo {
                current_version: current_version.to_string(),
                latest_version: latest_version.to_string(),
                release_url: release.html_url,
                release_notes: release.body,
            }))
        } else {
            Ok(None)
        }
    }

    /// Quick check with short timeout for startup
    pub async fn quick_update_check(&self) -> Option<UpdateInfo> {
        match tokio::time::timeout(UPDATE_CHECK_TIMEOUT, self.check_for_updates()).await {
            Ok(Ok(update_info)) => update_info,
            _ => None, // Silently fail on any error or timeout
        }
    }
}

#[derive(Debug)]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub release_url: String,
    pub release_notes: String,
}

impl UpdateInfo {
    pub fn display_notification(&self) -> String {
        format!(
            "📢 Update Available!\n\n\
            Current version: {}\n\
            Latest version: {}\n\n\
            Visit: {}\n\n\
            To update: cargo install --git https://github.com/{} --force\n\
            Or download from the release page above.\n\n\
            Press any key to continue...",
            self.current_version,
            self.latest_version,
            self.release_url,
            GITHUB_REPO
        )
    }

    pub fn short_notification(&self) -> String {
        format!(
            "New version {} available! Current: {} | Visit: {}",
            self.latest_version,
            self.current_version,
            self.release_url
        )
    }
} 