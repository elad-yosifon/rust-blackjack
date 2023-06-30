#[macro_export]
macro_rules! read_stdin_string {
    ($msg:expr) => {{
        println!("{}", $msg);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }};
}

#[macro_export]
macro_rules! read_stdin {
    ($msg:expr, $t:ty) => {{
        read_stdin_string!($msg).parse::<$t>().unwrap()
    }};
}

#[macro_export]
macro_rules! read_stdin_str {
    ($msg:expr) => {
        read_stdin_string!($msg).as_str()
    };
}

#[macro_export]
macro_rules! simulate_think {
    ($seconds:literal) => {
        sleep(Duration::from_secs($seconds));
    };
}

//TODO: deal to a hand
#[macro_export]
macro_rules! deal_card {
    ($player:expr, $deck:expr) => {
        $player.deal_card($deck.cards.pop_front().unwrap())
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
