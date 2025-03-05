#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    todo!("Implement uninstall service")
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
