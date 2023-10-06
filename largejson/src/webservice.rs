use std::path::Path;
use figment::{Figment, providers::{Yaml, Format}};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct WebServicePrefixConfig {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebServiceConfig {
    /// Prefix of the served API
    pub prefix: WebServicePrefixConfig,
}

#[derive(Deserialize, Debug)]
pub struct MyConfig {
    /// Config of my web service
    pub webservice: WebServiceConfig,
}

impl MyConfig {
    // Note the `nested` option on both `file` providers. This makes each
    // top-level dictionary act as a profile.
    pub fn figment<P: AsRef<Path>>(path: P) -> Figment {
        Figment::new().merge(Yaml::file(path))
    }
}
