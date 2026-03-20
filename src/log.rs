#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use std::io::{self, Write};
        let mut stdout = io::stdout();
        write!(stdout, "\x1b[34m[ INFO ]\x1b[0m ").unwrap();
        writeln!(stdout, "\x1b[34m{}\x1b[0m", format_args!($($arg)*)).unwrap();
    }};
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{
        use std::io::{self, Write};
        let mut stdout = io::stdout();
        write!(stdout, "\x1b[33m[ WARNING ]\x1b[0m ").unwrap();
        writeln!(stdout, "\x1b[33m{}\x1b[0m", format_args!($($arg)*)).unwrap();
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use std::io::{self, Write};
        let mut stdout = io::stdout();
        write!(stdout, "\x1b[31m[ ERROR ]\x1b[0m ").unwrap();
        writeln!(stdout, "\x1b[31m{}\x1b[0m", format_args!($($arg)*)).unwrap();
    }};
}

// gives file and line to find the issue faster
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        println!(
            "\x1b[32m[ DEBUG ]\x1b[0m [{}:{}] \x1b[32m{}\x1b[0m",
            file!(),
            line!(),
            format_args!($($arg)*)
        )
    };
}

// #[macro_export]
// macro_rules! debug {
//     ($($arg:tt)*) => {{
//         use std::io::{self, Write};
//         let mut stdout = io::stdout();
//         write!(stdout, "\x1b[32m[ WARNING ]\x1b[0m ").unwrap();
//         writeln!(stdout, "\x1b[32m{}\x1b[0m", format_args!($($arg)*)).unwrap();
//     }};
// }
