#[macro_export]
macro_rules! map {
    ($($key:expr => $value:expr),*) => {{
        let mut m = std::collections::HashMap::new();
        $(
            m.insert($key, $value);
        )*
        m
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        eprint!("\x1b[31merror\x1b[0m: ");
        eprintln!($($arg)*);
        std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! warn { // Unused
    ($($arg:tt)*) => {{
        eprint!("\x1b[33warning\x1b[0m: ");
        eprintln!($($arg)*);
    }};
}