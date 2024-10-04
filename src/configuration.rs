use std::str::FromStr;
use secrecy::{ExposeSecret, SecretBox};
use sqlx::postgres::{PgConnectOptions, PgSslMode};


// impl TryFrom<String> for Environment {
//     type Error = String;

//     fn try_from(s: String) -> Result<Self, Self::Error> {
//         Environment::from_str(&s)
//     }
// }
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    // pub application_port: u16
    pub application_settings: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub application_port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretBox<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Read the "default" config file
   settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;
    // settings = config::Config::builder()
    //     .add_source(config::File::from(configuration_directory.join("base")).required(true))
    //     .build()?;

    // Detect the running environment.
    // Default to `local` if unspecified
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
    .unwrap_or_else(|_| "local".into())
    .try_into()
    .expect("Failed to pass APP_ENVIRONMENT");

    settings.merge(config::File::from(configuration_directory.join(environment.as_str())).required(true));

    // let settings = config::Config::builder()
    //     .add_source(config::File::with_name("configuration"))
    //     .build()?;
    settings.try_deserialize()
}

pub enum Environment{
    Local,
    Production
}

impl Environment{
    pub fn as_str(&self) -> &'static str{
        match self {
            Environment::Local => "local",
            Environment::Production => "production"
        }

    }
}

impl TryFrom<String> for Environment{
    type Error = String;
    fn try_from(s:String) -> Result<Self, Self::Error>{
        match s.to_lowercase().as_str(){
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!("{} is not a supported environment. Use either `local` or `production`.", other))
        }
    }
}

impl DatabaseSettings {
    
    pub fn connection_string(&self) -> SecretBox<String> {
        SecretBox::new(Box::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        )))
    }

    pub fn connection_string_without_db(&self) -> SecretBox<String> {
        SecretBox::new(Box::new(format!(
            "postgres://{}:{}@{}:{}/",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        )))
    }
}
