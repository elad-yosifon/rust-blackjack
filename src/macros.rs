#[macro_export]
macro_rules! take_stdin_key {
    ($msg:expr $(,$char:literal)*) => {{
        println!("{}", $msg);
        use std::io::{stdin, stdout, Write};
        use termion::event::Key;
        use termion::input::TermRead;
        use termion::raw::IntoRawMode;

        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        let mut xc = '\0';
        for c in stdin.keys() {
            let x = match c.unwrap() {
                $(Key::Char($char) => $char,)*
                _ => '\0'
            };

            if x != '\0' {
                stdout.flush().unwrap();
                xc = x;
                break;
            }
        }
        xc.clone()
    }}
}

#[macro_export]
macro_rules! take_stdin_string {
    ($msg:expr, $take:literal) => {{
        println!("{}", $msg);
        use std::string::{String};
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.as_str().trim().to_string()
    }};
}

#[macro_export]
macro_rules! take_stdin {
    ($msg:expr, $t:ty, $take:literal) => {
        crate::take_stdin_string!($msg,$take).parse::<$t>().unwrap()
    };
}

#[macro_export]
macro_rules! take_stdin_str {
    ($msg:expr, $take:literal) => {
        crate::take_stdin_string!($msg).as_str()
    };
}

#[macro_export]
macro_rules! simulate_think {
    ($seconds:literal) => {
        std::thread::sleep(std::time::Duration::from_secs($seconds));
    };
}

#[macro_export]
macro_rules! at {
    ($vec:expr, $at:expr) => {
        $vec.get($at).unwrap()
    };
    (mut $vec:expr, $at:expr) => {
        $vec.get_mut($at).unwrap()
    };
}
