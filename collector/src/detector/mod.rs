mod analyzer;
/// Anomaly detection module
///
/// This module provides functionality for detecting anomalies in system metrics,
/// including rule-based detection and analysis helpers.
mod rules;

pub use analyzer::{calculate_delta, calculate_rate, classify_severity, MetricsDelta};
pub use rules::AnomalyRules;
