#![feature(exclusive_range_pattern)]

use std::{path, vec};
use std::cell::RefCell;
use std::ops::Deref;
use std::process::exit;
use std::rc::Rc;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, BlendMode, Color, DrawParam, Image};

use gameplay::round::blackjack_round;

use crate::gameplay::game::Game;
use crate::gfx::scenes::{Scene, Scenes, SceneType};

mod cards;
mod gameplay;
mod gfx;
mod macros;

fn main() {
    let resource_dir = /*if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {*/
        path::Path::new("/home/elad/IdeaProjects/rust-blackjack/resources/");
    // };

    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .resources_dir_name(resource_dir)
        .window_setup(WindowSetup::default().title("BlackJack"))
        .window_mode(WindowMode::default().dimensions(800.0, 800.0))
        .build()
        .expect("aieee, could not create ggez context!");

    // ctx.fs.print_all();
    let my_game = GameContext::new(&mut ctx).init();
    event::run(ctx, event_loop, my_game);
}

pub struct GameContext {
    board: Image,
    window_size: (f32, f32),
    scences: Scenes,
    current_scene: SceneType,
    game: Game,
}

impl GameContext {
    fn main_() {
        // let minimum_bet = 10;
        // let number_of_players = match take_stdin_key!("Number of players? [1/2/3]", '1', '2', '3') {
        //     '1' => 1,
        //     '2' => 2,
        //     '3' => 3,
        //     _ => 0,
        // };
        //
        // if !(1..=3).contains(&number_of_players) {
        //     println!("Game can start with 1-3 players");
        //     exit(-1);
        // } else {
        //     println!("{}", number_of_players);
        // }
        // // let number_of_players = 3;
        // let player_scores = vec![minimum_bet * 10; number_of_players];
        //
        // let player_names = vec![""; number_of_players]
        //     .iter()
        //     .enumerate()
        //     .map(|(idx, _)| format!("User_{}", idx + 1))
        //     .collect();
        //
        // let mut game = Game {
        //     number_of_players,
        //     player_scores,
        //     player_names,
        // };
        //
        // loop {
        //     let mut round = blackjack_round(number_of_players, minimum_bet);
        //     round.play();
        //
        //     simulate_think!(1);
        //     game.judge_round(&round);
        //
        //     simulate_think!(2);
        //     game.print_player_scores();
        //
        //     loop {
        //         println!();
        //         match take_stdin_key!("Another round? [y/n]:", 'y', 'n') {
        //             'y' => {
        //                 break;
        //             }
        //             'n' => {
        //                 println!("Thanks for playing, bye :)");
        //                 exit(0);
        //             }
        //             _ => {
        //                 unreachable!()
        //             }
        //         }
        //     }
        // }
    }

    pub fn new(_ctx: &mut Context) -> GameContext {
        let board = Image::from_path(_ctx, "/board.jpg").unwrap();
        GameContext {
            window_size: _ctx.gfx.size(),
            board,
            scences: Scenes::default(),
            current_scene: SceneType::ChooseNumberOfPlayers,
            game: Game::default(),
        }
    }

    fn init(mut self) -> GameContext {
        self.scences = Scenes::from_game(&self);
        self
    }

    pub fn start_game(&mut self, number_of_players: usize) {
        let minimum_bet = 10;

        if !(1..=3).contains(&number_of_players) {
            println!("Game can start with 1-3 players");
            exit(-1);
        } else {
            println!("Number of player: {}", number_of_players);
        }

        let player_scores = vec![minimum_bet * 10; number_of_players];

        let player_names = vec![""; number_of_players]
            .iter()
            .enumerate()
            .map(|(idx, _)| format!("User_{}", idx + 1))
            .collect();

        self.game = Game {
            number_of_players,
            player_scores,
            player_names,
        };
    }

    fn get_scene(&mut self) -> Rc<RefCell<dyn Scene>> {
        match self.current_scene {
            SceneType::ChooseNumberOfPlayers => self.scences.get(SceneType::ChooseNumberOfPlayers),
            SceneType::DealCards => self.scences.get(SceneType::DealCards),
            SceneType::PlayerTurn => unimplemented!(),
            SceneType::DealerTurn => unimplemented!(),
            SceneType::RoundSummary => unimplemented!(),
            SceneType::ChooseEndOfRoundAction => unimplemented!(),
        }
    }
}

impl EventHandler for GameContext {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.get_scene().borrow_mut().update(self, _ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_blend_mode(BlendMode::PREMULTIPLIED);
        canvas.draw(&self.board, DrawParam::default().scale(Vec2::new(2., 2.)));
        canvas.set_blend_mode(BlendMode::ALPHA);
        self.get_scene().borrow_mut().draw(ctx, &mut canvas);
        canvas.finish(ctx)
    }
}
