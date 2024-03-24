#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]

use merino::models::config::Config;
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
use tracing::{trace, error, warn, debug};

use merino::*;


/// Logo to be printed at when merino is run
const LOGO: &str = r"
                      _
  _ __ ___   ___ _ __(_)_ __   ___
 | '_ ` _ \ / _ \ '__| | '_ \ / _ \
 | | | | | |  __/ |  | | | | | (_) |
 |_| |_| |_|\___|_|  |_|_| |_|\___/

 A SOCKS5 Proxy server written in Rust
";
const CONFIG_FILE: &str = "config.yml";


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
    let config: Result::<Config, Box<dyn Error>> = if Path::new(CONFIG_FILE).exists(){ 
        debug!("Exists {CONFIG_FILE}");
        let config_file = PathBuf::from(CONFIG_FILE);
        let file = std::fs::File::open(&config_file).unwrap_or_else(|e| {
            error!("Can't open file {:?}: {}", &config_file, e);
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
                &config_file
            );
            std::process::exit(1);
        }else{
            debug!("Permissions are ok");
        }
        debug!("Loading config");
        let config: Config = serde_yaml::from_reader(file)
            .expect("Cant read config file");
        if !config.users.is_empty(){
            auth_methods.push(AuthMethods::UserPass as u8);
        }
        trace!("{:?}", config);
        Ok(config)
    }else{
        Ok(Config::default())
    };

    // Create proxy server
    let mut merino = Merino::new(port, &ip, auth_methods, config?, None).await?;

    // Start Proxies
    merino.serve().await;

    Ok(())
}
