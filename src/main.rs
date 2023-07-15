#![feature(exclusive_range_pattern)]

use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;
use std::{path, vec};

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, BlendMode, Color, DrawParam, Image};
use ggez::{Context, ContextBuilder, GameResult};

use crate::gameplay::game::Game;
use crate::gfx::scenes::{Scene, SceneType, Scenes};

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
        self.game = Game::init(number_of_players);
    }

    fn get_scene(&mut self) -> Rc<RefCell<dyn Scene>> {
        match self.current_scene {
            SceneType::ChooseNumberOfPlayers => self.scences.get(SceneType::ChooseNumberOfPlayers),
            SceneType::PlayRound => self.scences.get(SceneType::PlayRound),
            SceneType::RoundSummary => unimplemented!(),
            SceneType::EndOfRound => self.scences.get(SceneType::EndOfRound),
            SceneType::EndOfGame => self.scences.get(SceneType::EndOfGame),
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
