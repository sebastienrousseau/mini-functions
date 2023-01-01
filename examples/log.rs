use mini_functions::log::Log;

fn main() {
    let log = Log::new(
        "12345678-1234-1234-1234-1234567890ab",
        "2023-12-23T23:23:23.222222+00:00",
        "INFO",
        "SystemTrayEvent",
        "Showing main window",
    );
    log.log();
}
