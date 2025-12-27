// Module for Hybrid Network Sync
// Agents: Cloud_Architect, Security_Agent

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize)]
struct LicenseRequest {
    machine_id: String,
    plugin_version: String,
}

#[derive(Deserialize)]
struct LicenseResponse {
    valid: bool,
    offline_token: Option<String>,
}

pub struct CloudClient {
    client: Client,
    api_url: String,
}

impl CloudClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .user_agent("Antigravity-Hybrid/1.0")
            .build()
            .unwrap_or_default();

        Self {
            client,
            api_url: "https://api.antigravity.ai/v1".to_string(), // Placeholder prod URL
        }
    }

    /// specific check for the user's HWID against our Cloud Function
    pub async fn check_license(&self) -> bool {
        // In a real app, generate a stable HWID (e.g. from MAC addr or CPU ID)
        let machine_id = "stub-hardware-id-user-123".to_string();

        let request = LicenseRequest {
            machine_id,
            plugin_version: "0.1.0".to_string(),
        };

        // Attempt Network Verification
        match self.client
            .post(format!("{}/license/verify", self.api_url))
            .json(&request)
            .send()
            .await 
        {
            Ok(res) => {
                if let Ok(json) = res.json::<LicenseResponse>().await {
                    return json.valid;
                }
                false
            }
            Err(_) => {
                // FALLBACK: Offline Mode
                // If network fails, we default to TRUE for this demo (grace period)
                // In production, you'd check a locally cached crypto signature.
                println!("Network failed. Entering Offline Mode.");
                true 
            }
        }
    }

    pub async fn sync_presets(&self) {
        // Placeholder: Fetch presets from Firestore endpoint
        let _ = self.client.get(format!("{}/presets/sync", self.api_url)).send().await;
    }
}
