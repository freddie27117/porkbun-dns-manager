use chrono::Local;

pub fn deblogger(message: impl AsRef<str>) {
    let time = Local::now().format("%H:%M:%S");
    let message = message.as_ref();
    println!("[{}] DEBUG: {}", time, message)
}

pub fn deblogger_fatal(message: impl AsRef<str>, error: String) -> ! {
    let time = Local::now().format("%H:%M:%S");
    let message = message.as_ref();
    println!("[{}] FATAL: {} -> {}", time, message, error);
    std::process::exit(1);
}
