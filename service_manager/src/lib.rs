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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_toml_config() {
        let toml_str = r#"
            [service]
            name = "TestService"
            display_name = "Test Service"
            description = "A test service"

            [startup]
            executable_path = "C:\\test\\service.exe"
            arguments = ["--config", "conf.toml"]
            start_type = "Automatic"
            dependencies = ["Dependency1", "Dependency2"]
            error_control = "Normal"

            [credentials]
            username = "TestUser"
            password = "TestPass"
        "#;

        let config: ServiceConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.service.name, "TestService");
        assert_eq!(config.service.display_name, "Test Service");
        assert_eq!(
            config.service.description,
            Some("A test service".to_string())
        );
        assert_eq!(config.startup.executable_path, "C:\\test\\service.exe");
        assert_eq!(config.startup.arguments, vec!["--config", "conf.toml"]);
        assert_eq!(config.startup.start_type, "Automatic");
        assert_eq!(
            config.startup.dependencies,
            vec!["Dependency1", "Dependency2"]
        );
        assert_eq!(config.startup.error_control, "Normal");
        assert_eq!(config.credentials.as_ref().unwrap().username, "TestUser");
        assert_eq!(config.credentials.as_ref().unwrap().password, "TestPass");
    }
}
