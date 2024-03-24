#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]

use merino::*;
use tracing_subscriber::Layer;
use std::env;
use std::error::Error;
use std::os::unix::prelude::MetadataExt;
use std::path::{PathBuf, Path};
use std::str::FromStr;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{
    layer::SubscriberExt,
    EnvFilter};
use tracing::{error, warn, debug};


/// Logo to be printed at when merino is run
const LOGO: &str = r"
                      _
  _ __ ___   ___ _ __(_)_ __   ___
 | '_ ` _ \ / _ \ '__| | '_ \ / _ \
 | | | | | |  __/ |  | | | | | (_) |
 |_| |_| |_|\___|_|  |_|_| |_|\___/

 A SOCKS5 Proxy server written in Rust
";
const USERS_FILE: &str = "users.yml";


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", LOGO);

    let format = time::format_description::parse(
        "[year]-[month padding:zero]-[day padding:zero]T[hour]:[minute]:[second]",
    ).expect("Can't parse timer");
    let offset_in_sec = chrono::Local::now()
        .offset()
        .local_minus_utc();
    let time_offset = time::UtcOffset::from_whole_seconds(offset_in_sec).unwrap();

    let timer = tracing_subscriber::fmt::time::OffsetTime::new(time_offset, format);
    let log_level = env::var("RUST_LOG")
        .unwrap_or("DEBUG".to_string());
    let log_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_timer(timer)
        .with_thread_names(true)
        .with_filter(EnvFilter::from_str(&log_level).unwrap());


    tracing_subscriber::registry()
        .with(log_layer)
        .init();

    warn!("Log level: {:?}", log_level);


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
    let authed_users: Result<Vec<User>, Box<dyn Error>> = if Path::new(USERS_FILE).exists(){
        debug!("Exists {USERS_FILE}");
        let users_file = PathBuf::from(USERS_FILE);
        auth_methods.push(AuthMethods::UserPass as u8);
        let file = std::fs::File::open(&users_file).unwrap_or_else(|e| {
            error!("Can't open file {:?}: {}", &users_file, e);
            std::process::exit(1);
        });
        debug!("Metadata");

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
        }else{
            debug!("Okis");
        }

        debug!("Going to load users");
        let users: Vec<User>  = serde_yaml::from_reader(file)
            .expect("Cant read users file");
        debug!("{:?}", users);
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
