use serde::Deserialize;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
#[cfg(windows)]
use windows_service::service::{
    ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType,
};
#[cfg(windows)]
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

#[cfg(windows)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to parse service start type")]
    ParseErrorServiceStartType,
    #[error("Failed to parse service error control")]
    ParseErrorServiceErrorControl,
    #[error("Failed to parse service type")]
    ParseErrorServiceType,
    #[error("Internal windows-service crate error")]
    WindowsServiceError(#[from] windows_service::Error),
}

#[cfg(windows)]
type Result<T> = std::result::Result<T, Error>;

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
    /// Valid values are:
    /// * auto_start
    //  * on_demand
    //  * disabled
    //  * system_start
    //  * boot_start
    pub start_type: String,
    /// Dependencies of the service
    pub dependencies: Vec<String>,
    /// How service errors are handled
    /// Valid values are:
    /// * ignore
    /// * normal
    /// * severe
    /// * critical
    pub error_control: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Credentials {
    /// Account name used to run the service
    pub username: String,
    /// Account password
    pub password: String,
}

/// Converts a string representation of a service type to `ServiceType`.
///
/// # Arguments
///
/// * `service_type` - A string slice that holds the service type name
///
/// # Returns
///
/// * `Ok(ServiceType)` - The corresponding `ServiceType` enum variant
/// * `Err(Error::ParseErrorServiceType)` - If the input string doesn't match any known service type
///
/// The function accepts the following values (case-insensitive):
/// * "file_system_driver" -> ServiceType::FILE_SYSTEM_DRIVER
/// * "kernel_driver" -> ServiceType::KERNEL_DRIVER
/// * "own_process" -> ServiceType::OWN_PROCESS
/// * "share_process" -> ServiceType::SHARE_PROCESS
/// * "user_own_process" -> ServiceType::USER_OWN_PROCESS
/// * "user_share_process" -> ServiceType::USER_SHARE_PROCESS
/// * "interactive_process" -> ServiceType::INTERACTIVE_PROCESS
#[cfg(windows)]
fn try_into_service_type<S: AsRef<str>>(service_type: S) -> Result<ServiceType> {
    let s = service_type.as_ref().to_lowercase();
    match s.as_str() {
        "file_system_driver" => Ok(ServiceType::FILE_SYSTEM_DRIVER),
        "kernel_driver" => Ok(ServiceType::KERNEL_DRIVER),
        "own_process" => Ok(ServiceType::OWN_PROCESS),
        "share_process" => Ok(ServiceType::SHARE_PROCESS),
        "user_own_process" => Ok(ServiceType::USER_OWN_PROCESS),
        "user_share_process" => Ok(ServiceType::USER_SHARE_PROCESS),
        "interactive_process" => Ok(ServiceType::INTERACTIVE_PROCESS),
        _ => {
            tracing::error!("Unknown service type: {}", service_type.as_ref());
            Err(Error::ParseErrorServiceType)
        }
    }
}

/// Converts a string representation of a service start type to `ServiceStartType`.
///
/// # Arguments
///
/// * `start_type` - A string slice that holds the service start type name
///
/// # Returns
///
/// * `Ok(ServiceStartType)` - The corresponding `ServiceStartType` enum variant
/// * `Err(Error::ParseErrorServiceStartType)` - If the input string doesn't match any known start type
///
/// The function accepts the following values (case-insensitive):
/// * "auto_start" -> ServiceStartType::AutoStart
/// * "on_demand" -> ServiceStartType::OnDemand
/// * "disabled" -> ServiceStartType::Disabled
/// * "system_start" -> ServiceStartType::SystemStart
/// * "boot_start" -> ServiceStartType::BootStart
fn try_into_service_start_type<S: AsRef<str>>(start_type: S) -> Result<ServiceStartType> {
    let s = start_type.as_ref().to_lowercase();
    match s.as_str() {
        "auto_start" => Ok(ServiceStartType::AutoStart),
        "on_demand" => Ok(ServiceStartType::OnDemand),
        "disabled" => Ok(ServiceStartType::Disabled),
        "system_start" => Ok(ServiceStartType::SystemStart),
        "boot_start" => Ok(ServiceStartType::BootStart),
        _ => {
            tracing::error!("Unknown service start type: {}", start_type.as_ref());
            Err(Error::ParseErrorServiceStartType)
        }
    }
}

/// Converts a string representation of a service error control to `ServiceErrorControl`.
///
/// # Arguments
///
/// * `error_control` - A string slice that holds the service error control name
///
/// # Returns
///
/// * `Ok(ServiceErrorControl)` - The corresponding `ServiceErrorControl` enum variant
/// * `Err(Error::ParseErrorServiceErrorControl)` - If the input string doesn't match any known error control
///
/// The function accepts the following values (case-insensitive):
/// * "critical" -> ServiceErrorControl::Critical
/// * "normal" -> ServiceErrorControl::Normal
/// * "severe" -> ServiceErrorControl::Severe
/// * "ignore" -> ServiceErrorControl::Ignore
#[cfg(windows)]
fn try_into_service_error_control<S: AsRef<str>>(error_control: S) -> Result<ServiceErrorControl> {
    let s = error_control.as_ref().to_lowercase();
    match s.as_str() {
        "critical" => Ok(ServiceErrorControl::Critical),
        "normal" => Ok(ServiceErrorControl::Normal),
        "severe" => Ok(ServiceErrorControl::Severe),
        "ignore" => Ok(ServiceErrorControl::Ignore),
        _ => {
            tracing::error!("Unknown service error control: {}", error_control.as_ref());
            Err(Error::ParseErrorServiceErrorControl)
        }
    }
}

/// Creates a new Windows service on the local machine using the provided configuration.
///
/// # Arguments
///
/// * `config` - A reference to `ServiceConfig` containing all the necessary service configuration parameters
///   including service name, display name, description, startup parameters, and optional credentials.
///
/// # Returns
///
/// * `Ok(())` - If the service was successfully created
/// * `Err(Error)` - If the service creation failed, with specific error details:
///   - `ParseErrorServiceStartType` if the start type string is invalid
///   - `ParseErrorServiceErrorControl` if the error control string is invalid
///   - `ParseErrorServiceType` if the service type string is invalid
///   - `WindowsServiceError` for other Windows service-related errors
///
/// # Platform Specific
///
/// This function is only available on Windows platforms.
#[cfg(windows)]
pub fn create_local_service(config: &ServiceConfig) -> Result<()> {
    let service_type = try_into_service_type(&config.startup.start_type)?;
    let start_type = try_into_service_start_type(&config.startup.start_type)?;
    let error_control = try_into_service_error_control(&config.startup.error_control)?;

    let service_info = ServiceInfo {
        name: OsString::from(&config.service.name),
        display_name: OsString::from(&config.service.display_name),
        service_type,
        start_type,
        error_control,
        executable_path: PathBuf::from(&config.startup.executable_path.as_str()),
        launch_arguments: config
            .startup
            .arguments
            .iter()
            .map(OsString::from)
            .collect(),
        dependencies: vec![],
        account_name: config
            .credentials
            .as_ref()
            .map(|credentials| OsString::from(&credentials.username)),
        account_password: config
            .credentials
            .as_ref()
            .map(|credentials| OsString::from(&credentials.password)),
    };
    let request_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = ServiceManager::local_computer(None::<&str>, request_access)?;
    let service = service_manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG)?;
    if let Some(dependencies) = config.service.description.as_ref() {
        service.set_description(OsStr::new(dependencies.as_str()))?;
    }
    Ok(())
}

#[cfg(test)]
#[cfg(windows)]
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

    #[test]
    fn test_service_type_file_system_driver() {
        assert_eq!(
            try_into_service_type("file_system_driver").unwrap(),
            ServiceType::FILE_SYSTEM_DRIVER
        );
    }

    #[test]
    fn test_service_type_kernel_driver() {
        assert_eq!(
            try_into_service_type("kernel_driver").unwrap(),
            ServiceType::KERNEL_DRIVER
        );
    }

    #[test]
    fn test_service_type_own_process() {
        assert_eq!(
            try_into_service_type("own_process").unwrap(),
            ServiceType::OWN_PROCESS
        );
    }

    #[test]
    fn test_service_type_share_process() {
        assert_eq!(
            try_into_service_type("share_process").unwrap(),
            ServiceType::SHARE_PROCESS
        );
    }

    #[test]
    fn test_service_type_user_own_process() {
        assert_eq!(
            try_into_service_type("user_own_process").unwrap(),
            ServiceType::USER_OWN_PROCESS
        );
    }

    #[test]
    fn test_service_type_user_share_process() {
        assert_eq!(
            try_into_service_type("user_share_process").unwrap(),
            ServiceType::USER_SHARE_PROCESS
        );
    }

    #[test]
    fn test_service_type_interactive_process() {
        assert_eq!(
            try_into_service_type("interactive_process").unwrap(),
            ServiceType::INTERACTIVE_PROCESS
        );
    }

    #[test]
    fn test_service_type_invalid() {
        assert!(try_into_service_type("invalid").is_err());
    }

    #[test]
    fn test_service_start_type_auto_start() {
        assert_eq!(
            try_into_service_start_type("auto_start").unwrap(),
            ServiceStartType::AutoStart
        );
    }

    #[test]
    fn test_service_start_type_on_demand() {
        assert_eq!(
            try_into_service_start_type("on_demand").unwrap(),
            ServiceStartType::OnDemand
        );
    }

    #[test]
    fn test_service_start_type_disabled() {
        assert_eq!(
            try_into_service_start_type("disabled").unwrap(),
            ServiceStartType::Disabled
        );
    }

    #[test]
    fn test_service_start_type_system_start() {
        assert_eq!(
            try_into_service_start_type("system_start").unwrap(),
            ServiceStartType::SystemStart
        );
    }

    #[test]
    fn test_service_start_type_boot_start() {
        assert_eq!(
            try_into_service_start_type("boot_start").unwrap(),
            ServiceStartType::BootStart
        );
    }

    #[test]
    fn test_service_start_type_invalid_start_type() {
        assert!(try_into_service_start_type("invalid").is_err());
    }

    #[test]
    fn test_service_error_control_critical_error_control() {
        assert_eq!(
            try_into_service_error_control("critical").unwrap(),
            ServiceErrorControl::Critical
        );
    }

    #[test]
    fn test_service_error_control_normal_error_control() {
        assert_eq!(
            try_into_service_error_control("normal").unwrap(),
            ServiceErrorControl::Normal
        );
    }

    #[test]
    fn test_service_error_control_severe_error_control() {
        assert_eq!(
            try_into_service_error_control("severe").unwrap(),
            ServiceErrorControl::Severe
        );
    }

    #[test]
    fn test_service_error_control_ignore_error_control() {
        assert_eq!(
            try_into_service_error_control("ignore").unwrap(),
            ServiceErrorControl::Ignore
        );
    }

    #[test]
    fn test_service_error_control_invalid_error_control() {
        assert!(try_into_service_error_control("invalid").is_err());
    }
}
