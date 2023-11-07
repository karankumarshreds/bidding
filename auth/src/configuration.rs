#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
    pub jwt: JwtSettings,
}

impl TryFrom<config::Config> for Settings {
    type Error = config::ConfigError;
    fn try_from(value: config::Config) -> Result<Self, Self::Error> {
        let application_port = value.get::<u16>("application_port")?;
        let database = value.get::<DatabaseSettings>("database")?;
        let jwt = value.get::<JwtSettings>("jwt")?;
        Ok(
            Settings {
                application_port,
                database,
                jwt,
            }
        )
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub db_name: String,
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct JwtSettings {
    pub secret: String,
    pub expiration: i32,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.db_name,
        )
    }
    pub fn connection_string_without_db_name(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password,
            self.host,
            self.port,
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize our configuration reader
    let settings_builder = config::Config::builder();
    let settings: Settings = settings_builder
        .add_source(config::File::with_name("configuration"))
        .build()?
        .try_into()?;
    Ok(settings)
}

