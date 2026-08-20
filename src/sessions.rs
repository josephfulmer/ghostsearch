use anyhow::Result;
use rand::{distr::Alphanumeric, Rng};

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,

    // Deliberately random and ephemeral.
    //
    // This value is supplied to Tor as the SOCKS5 authentication
    // isolation value. Sessions with different values are kept
    // on separate Tor circuits when isolation is enabled.
    pub isolation_token: String,
}

impl Session {
    pub fn new() -> Self {
        let id = uuid::Uuid::new_v4().to_string();

        let isolation_token: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        Self {
            id,
            isolation_token,
        }
    }

    pub fn destroy(self) {
        // The important part of destruction is that the token is
        // no longer retained by the application.
        //
        // A future version can additionally ask Tor to tear down
        // the corresponding circuits.
        drop(self);
    }
}

pub fn create_session() -> Result<Session> {
    Ok(Session::new())
}
