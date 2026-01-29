use anyhow::{Context, Result};
use reqwest::Client;
use shared::types::{Anomaly, SystemMetrics};
use std::time::Duration;

/// API client for communicating with the system-monitor server
pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(base_url: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client, base_url })
    }

    /// Get current system metrics
    pub async fn get_current_metrics(&self) -> Result<SystemMetrics> {
        let url = format!("{}/api/metrics/current", self.base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send request to /api/metrics/current")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Server returned error status: {} for /api/metrics/current",
                response.status()
            );
        }

        let metrics = response
            .json::<SystemMetrics>()
            .await
            .context("Failed to parse metrics JSON response")?;

        Ok(metrics)
    }

    /// Get recent anomalies
    pub async fn get_anomalies(&self, limit: Option<usize>) -> Result<Vec<Anomaly>> {
        let mut url = format!("{}/api/anomalies", self.base_url);

        if let Some(limit) = limit {
            url = format!("{}?limit={}", url, limit);
        }

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send request to /api/anomalies")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Server returned error status: {} for /api/anomalies",
                response.status()
            );
        }

        let anomalies = response
            .json::<Vec<Anomaly>>()
            .await
            .context("Failed to parse anomalies JSON response")?;

        Ok(anomalies)
    }

    /// Get historical metrics
    #[allow(dead_code)]
    pub async fn get_history(
        &self,
        start: Option<chrono::DateTime<chrono::Utc>>,
        end: Option<chrono::DateTime<chrono::Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<SystemMetrics>> {
        let mut url = format!("{}/api/metrics/history", self.base_url);
        let mut params = vec![];

        if let Some(start) = start {
            params.push(format!("start={}", start.to_rfc3339()));
        }

        if let Some(end) = end {
            params.push(format!("end={}", end.to_rfc3339()));
        }

        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send request to /api/metrics/history")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Server returned error status: {} for /api/metrics/history",
                response.status()
            );
        }

        let history = response
            .json::<Vec<SystemMetrics>>()
            .await
            .context("Failed to parse history JSON response")?;

        Ok(history)
    }

    /// Check if the server is healthy
    #[allow(dead_code)]
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/health", self.base_url);

        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = ApiClient::new("http://localhost:8080".to_string());
        assert!(client.is_ok());
    }
}
