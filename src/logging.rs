#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        eprintln!("{} {}", "error:".red().bold(), format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        eprintln!("{} {}", "warning:".yellow().bold(), format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("{} {}", "info:".blue().bold(), format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_success {
    ($($arg:tt)*) => {
        println!("{} {}", "info:".green().bold(), format_args!($($arg)*));
    };
}
