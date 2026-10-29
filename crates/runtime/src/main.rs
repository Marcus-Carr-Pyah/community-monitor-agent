use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use monitor_agent_http_executor::validate_check;
use monitor_agent_protocol::{AgentMode, HttpRequestSpec};
use monitor_agent_policy::validate_url_syntax;
use url::Url;

#[derive(Debug, Parser)]
#[command(name = "monitor-agent", version, about = "Community Monitor Rust agent")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Validate URL syntax and baseline public-agent policy without making a request.
    ValidateUrl {
        /// URL to validate.
        url: String,
    },
    /// Print a warning that this scaffold does not yet connect to a control plane.
    Status,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::ValidateUrl { url } => {
            let parsed = Url::parse(&url).context("URL could not be parsed")?;
            let request = HttpRequestSpec {
                method: "GET".to_owned(),
                url: parsed.clone(),
            };
            validate_url_syntax(&parsed, AgentMode::Public)
                .context("URL rejected by baseline public policy")?;
            validate_check(&request).context("request rejected by baseline policy")?;
            println!("URL passed baseline syntax checks. No network request was made.");
        }
        Command::Status => {
            println!("Community Monitor Agent scaffold");
            println!("Control-plane polling is not implemented yet.");
            println!("Do not use this build to execute untrusted jobs.");
        }
    }
    Ok(())
}
