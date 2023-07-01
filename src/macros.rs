#[macro_export]
macro_rules! read_stdin_string {
    ($msg:expr) => {{
        use std::string::{String, ToString};
        println!("{}", $msg);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }};
}

#[macro_export]
macro_rules! read_stdin {
    ($msg:expr, $t:ty) => {{
        crate::read_stdin_string!($msg).parse::<$t>().unwrap()
    }};
}

#[macro_export]
macro_rules! read_stdin_str {
    ($msg:expr) => {
        crate::read_stdin_string!($msg).as_str()
    };
}

#[macro_export]
macro_rules! simulate_think {
    ($seconds:literal) => {
        std::thread::sleep(std::time::Duration::from_secs($seconds));
    };
}

#[macro_export]
macro_rules! draw_card {
    ($deck:expr) => {
        $deck.cards.pop_front().unwrap()
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

#[macro_export]
macro_rules! actor_at {
    ($actors:expr, $at:expr) => {
        $actors.get($at).unwrap()
    };
    (mut $actors:expr, $at:expr) => {
        $actors.get_mut($at).unwrap()
    };
}
