use anyhow::Result;
use std::time::Duration;

/// Configuration for the TUI client
#[derive(Debug, Clone)]
pub struct Config {
    /// API base URL (default: http://localhost:8080)
    pub api_url: String,
    /// Refresh rate for metrics polling (default: 2 seconds)
    pub refresh_rate: Duration,
    /// Enable WebSocket streaming mode (default: false, uses HTTP polling)
    pub use_websocket: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: "http://localhost:8080".to_string(),
            refresh_rate: Duration::from_secs(2),
            use_websocket: false,
        }
    }
}

impl Config {
    /// Parse configuration from command line arguments
    pub fn from_args() -> Result<Self> {
        let mut config = Config::default();
        let args: Vec<String> = std::env::args().collect();

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--api-url" | "-u" => {
                    if i + 1 < args.len() {
                        config.api_url = args[i + 1].clone();
                        i += 2;
                    } else {
                        anyhow::bail!("Missing value for {}", args[i]);
                    }
                }
                "--refresh" | "-r" => {
                    if i + 1 < args.len() {
                        let seconds: u64 = args[i + 1].parse().map_err(|_| {
                            anyhow::anyhow!("Invalid refresh rate: {}", args[i + 1])
                        })?;
                        config.refresh_rate = Duration::from_secs(seconds);
                        i += 2;
                    } else {
                        anyhow::bail!("Missing value for {}", args[i]);
                    }
                }
                "--websocket" | "-w" => {
                    config.use_websocket = true;
                    i += 1;
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                _ => {
                    anyhow::bail!("Unknown argument: {}", args[i]);
                }
            }
        }

        Ok(config)
    }
}

fn print_help() {
    println!("System Monitor TUI Client");
    println!();
    println!("USAGE:");
    println!("    tui-client [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -u, --api-url <URL>        API base URL (default: http://localhost:8080)");
    println!("    -r, --refresh <SECONDS>    Refresh rate in seconds (default: 2)");
    println!("    -w, --websocket            Use WebSocket streaming instead of HTTP polling");
    println!("    -h, --help                 Print help information");
    println!();
    println!("EXAMPLES:");
    println!("    tui-client");
    println!("    tui-client --api-url http://192.168.1.100:8080 --refresh 5");
    println!("    tui-client -w -r 1");
    println!();
    println!("CONTROLS:");
    println!("    q, Ctrl+C                  Quit application");
    println!("    Up/Down, k/j              Scroll anomalies list");
    println!("    Page Up/Down              Scroll anomalies page by page");
}
