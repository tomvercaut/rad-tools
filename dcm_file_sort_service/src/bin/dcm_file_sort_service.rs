#[cfg(windows)]
fn main() {
    service::run().expect("Failed to run service");
}

#[cfg(not(windows))]
fn main() {
    panic!("This service is only intended to run on Windows");
}

#[cfg(windows)]
mod service {
    use rad_tools_dcm_file_sort_service::service::my_service_main;
    use windows_service::define_windows_service;

    pub fn run() -> windows_service::Result<()> {
        windows_service::service_dispatcher::start(
            rad_tools_dcm_file_sort_service::service::NAME,
            ffi_service_main,
        )
    }

    define_windows_service!(ffi_service_main, my_service_main);
}
