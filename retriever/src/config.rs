use anyhow::{anyhow, bail, Result};
use config::ConfigError::NotFound;

mod cli {
    use clap::{arg, command, Command};

    pub fn cli_app() -> Command {
        command!()
            .arg(arg!(-c --config <FILE> "Sets a custom config file"))
            .allow_external_subcommands(true)
    }
}

struct RawConfig(config::Config);

impl RawConfig {
    fn get_string(&self, key: &'static str) -> Result<String> {
        self.0
            .get_string(key)
            .map_err(|e| anyhow!("Cannot parse config key `{}` as string: {:?}", key, e))
    }

    fn get_u64_opt(&self, key: &'static str) -> Result<Option<u64>> {
        match self.0.get_int(key) {
            Ok(x) => Ok(Some(x as u64)),
            Err(NotFound(_)) => Ok(None),
            Err(e) => Err(anyhow!("Cannot parse config key `{}` as int: {:?}", key, e)),
        }
    }
}

pub struct Config {
    pub log_level: String,
    pub eth_rpc_url: String,
    pub grpc_listen_address: String,
    pub max_ongoing_retrieve_request: Option<u64>,
}

impl Config {
    pub fn from_cli_file() -> Result<Self> {
        let matches = cli::cli_app().get_matches();
        let c = if let Some(config_file) = matches.get_one::<String>("config") {
            RawConfig(
                config::Config::builder()
                    .add_source(config::File::with_name(&config_file))
                    .build()?,
            )
        } else {
            bail!(anyhow!("Config file missing!"));
        };

        Ok(Self {
            log_level: c.get_string("log_level")?,
            eth_rpc_url: c.get_string("eth_rpc_endpoint")?,
            grpc_listen_address: c.get_string("grpc_listen_address")?,
            max_ongoing_retrieve_request: c.get_u64_opt("max_ongoing_retrieve_request")?,
        })
    }
}
