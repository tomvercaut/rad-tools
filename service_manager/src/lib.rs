use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ServiceConfig {
    pub service: Service,
    pub startup: Startup,
    pub credentials: Option<Credentials>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Service {
    /// Name of the service
    pub name: String,
    /// Display name of the service
    pub display_name: String,
    /// Description on what the service does
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Startup {
    /// Path to the Windows service executable
    pub executable_path: String,
    /// Launch arguments of the service
    pub arguments: Vec<String>,
    /// How the service is started: automatically, on demand, disabled ...
    pub start_type: String,
    /// Dependencies of the service
    pub dependencies: Vec<String>,
    /// How service errors are handled
    pub error_control: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Credentials {
    /// Account name used to run the service
    pub username: String,
    /// Account password
    pub password: String,
}
