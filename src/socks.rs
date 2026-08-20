use anyhow::{Context, Result};
use reqwest::{Client, Proxy};

use crate::session::Session;

/// Build an HTTP client whose traffic goes through Tor.
///
/// Tor's SOCKS interface accepts the username/password fields as
/// stream-isolation parameters. We put the random session token
/// in the password field.
///
/// IMPORTANT:
/// - URLs are handed to the SOCKS proxy as hostnames.
/// - We do not resolve them ourselves.
/// - No direct HTTP client is created as a fallback.
pub fn client_for_session(
    session: &Session,
    socks_addr: &str,
) -> Result<Client> {
    let proxy_url = format!(
        "socks5h://{}:{}@{}",
        "tor-research",
        session.isolation_token,
        socks_addr
    );

    let proxy = Proxy::all(&proxy_url)
        .context("failed to configure Tor SOCKS proxy")?;

    Client::builder()
        .proxy(proxy)
        .redirect(reqwest::redirect::Policy::limited(10))
        .user_agent("Mozilla/5.0")
        .build()
        .context("failed to build HTTP client")
}
