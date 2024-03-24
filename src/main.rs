#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]

use merino::*;
use std::env;
use std::error::Error;
use std::os::unix::prelude::MetadataExt;
use std::path::{PathBuf, Path};
use std::str::FromStr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing::{trace, error, warn};

/// Logo to be printed at when merino is run
const LOGO: &str = r"
                      _
  _ __ ___   ___ _ __(_)_ __   ___
 | '_ ` _ \ / _ \ '__| | '_ \ / _ \
 | | | | | |  __/ |  | | | | | (_) |
 |_| |_| |_|\___|_|  |_|_| |_|\___/

 A SOCKS5 Proxy server written in Rust
";


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", LOGO);
    let log_level = env::var("RUST_LOG").unwrap_or("DEBUG".to_string());

    warn!("Log level: {:?}", log_level);

    tracing_subscriber::registry()
        .with(EnvFilter::from_str(&log_level).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let ip = env::var("IP").unwrap_or("0.0.0.0".to_string());
    let port:u16 = env::var("PORT")
        .unwrap_or("1080".to_string())
        .parse()
        .unwrap_or(1080);
    let allow_insecure = env::var("ALLOW_INSECURE")
        .unwrap_or("false".to_string())
        .to_lowercase() == "true";

    let no_auth = env::var("NO_AUTH")
        .unwrap_or("false".to_string())
        .to_lowercase() == "true";


    // Setup Proxy settings
    let mut auth_methods: Vec<u8> = Vec::new();
    // Allow unauthenticated connections
    if no_auth {
        auth_methods.push(merino::AuthMethods::NoAuth as u8);
    }
    // Enable username/password auth
    let authed_users: Result<Vec<User>, Box<dyn Error>> = if Path::new("/app/users.csv").exists(){
        let users_file = PathBuf::from("/app/users.csv");
        auth_methods.push(AuthMethods::UserPass as u8);
        let file = std::fs::File::open(&users_file).unwrap_or_else(|e| {
            error!("Can't open file {:?}: {}", &users_file, e);
            std::process::exit(1);
        });

        let metadata = file.metadata()?;
        // 7 is (S_IROTH | S_IWOTH | S_IXOTH) or the "permisions for others" in unix
        if (metadata.mode() & 7) > 0 && allow_insecure {
            error!(
                "Permissions {:o} for {:?} are too open. \
                It is recommended that your users file is NOT accessible by others. \
                To override this check, set --allow-insecure",
                metadata.mode() & 0o777,
                &users_file
            );
            std::process::exit(1);
        }

        let mut users: Vec<User> = Vec::new();

        let mut rdr = csv::Reader::from_reader(file);
        for result in rdr.deserialize() {
            let record: User = match result {
                Ok(r) => r,
                Err(e) => {
                    error!("{}", e);
                    std::process::exit(1);
                }
            };

            trace!("Loaded user: {}", record.username);
            users.push(record);
        }
        Ok(users)
    }else{
        Ok(Vec::new())
    };


    let authed_users = authed_users?;

    // Create proxy server
    let mut merino = Merino::new(port, &ip, auth_methods, authed_users, None).await?;

    // Start Proxies
    merino.serve().await;

    Ok(())
}
