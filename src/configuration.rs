use std::{str::FromStr, io::{Read, BufReader}, fs::File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) address: String,
    pub(crate) port: u16,
}

impl TryFrom<File> for Config {
    type Error = crate::Error;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        let reader = BufReader::new(value);
        reader.try_into()
    }
}

impl<R> TryFrom<BufReader<R>> for Config
where R: Read {
        type Error = crate::Error;

        fn try_from(mut value: BufReader<R>) -> Result<Self, Self::Error> {
            let mut s = String::new();
            value.read_to_string(&mut s)?;
            
            toml::from_str(&*s).map_err(|e| {
                tracing::error!(error = %e, "invalid TOML configration file");
                Box::new(e) as Self::Error
            })
        }
}


impl FromStr for Config {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s).map_err(|e| {
            tracing::error!(error = %e, "invalid TOML configration file");
            Box::new(e) as Self::Err
        })
    }
}


impl Default for Config {
    fn default() -> Self {
        Self {
            address: "0.0.0.0".to_string(),
            port: 3000,
        }
    }
}
