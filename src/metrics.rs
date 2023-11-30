// metrics.rs

use prometheus::{register_counter, Counter, Encoder, TextEncoder};

// Public function to initialize the Prometheus counter
pub fn initialize_counter() -> Result<Counter, String> {
    register_counter!("my_counter", "A counter for demonstration")
        .map_err(|e| format!("Failed to register 'my_counter': {}", e))
}

// Public function to serve the Prometheus metrics
pub async fn metrics_handler() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

