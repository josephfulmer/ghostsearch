mod session;
mod socks;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "tor-research",
    about = "Ephemeral, isolated research sessions over Tor"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Tor SOCKS address.
    #[arg(long, default_value = "127.0.0.1:9050")]
    socks: String,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new isolated research session.
    New,

    /// Test a URL through a new isolated session.
    Fetch {
        url: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::New => {
            let session = session::create_session()?;

            println!("session: {}", session.id);
            println!("isolation: {}", session.isolation_token);

            println!();
            println!(
                "This session is ephemeral. Destroy it when research is complete."
            );
        }

        Command::Fetch { url } => {
            let session = session::create_session()?;

            println!("session: {}", session.id);

            let client = socks::client_for_session(
                &session,
                &cli.socks,
            )?;

            let response = client
                .get(&url)
                .send()
                .await
                .context("request through Tor failed")?;

            println!("status: {}", response.status());

            let body = response
                .text()
                .await
                .context("failed reading response")?;

            println!("{}", body);

            session.destroy();
        }
    }

    Ok(())
}
