#![feature(exclusive_range_pattern)]

use std::cell::RefCell;
use std::ops::{AddAssign, SubAssign};
use std::path;
use std::rc::Rc;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, BlendMode, Color, Drawable, DrawParam, Image};

use gfx::elements::cards_sprite::CardsSprite;

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
    let my_game = GameContext::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}

pub struct GameContext {
    board: Image,
    cards_sprite: CardsSprite,
    window_size: (f32, f32),
    scences: Scenes,
    current_scene: SceneType,
    game: Game,
    // pub x: u32,
    // pub y: u32,
    // pub w: u32,
    // pub h: u32,
    // pub cards: Image,
}

impl GameContext {
    pub fn new(_ctx: &mut Context) -> GameContext {
        GameContext {
            window_size: _ctx.gfx.size(),
            board: Image::from_path(_ctx, "/board.jpg").unwrap(),
            cards_sprite: CardsSprite::new(_ctx),
            scences: Scenes::from_context(_ctx),
            current_scene: SceneType::ChooseNumberOfPlayers,
            game: Game::default(),
        }
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
        // if _ctx.keyboard.is_key_pressed(VirtualKeyCode::Left) {
        //     self.x.sub_assign(195);
        // }
        // else if _ctx.keyboard.is_key_pressed(VirtualKeyCode::Right) {
        //     self.x.add_assign(195);
        // }
        // else if _ctx.keyboard.is_key_pressed(VirtualKeyCode::Down) {
        //     self.y.add_assign(285);
        // }
        // else if _ctx.keyboard.is_key_pressed(VirtualKeyCode::Up) {
        //     self.y.sub_assign(285);
        // }
        // sleep(Duration::from_millis(100));
        self.get_scene().borrow_mut().update(self, _ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // println!("FPS: {}", ctx.time.fps());
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.set_blend_mode(BlendMode::PREMULTIPLIED);
        canvas.draw(&self.board, DrawParam::default().scale(Vec2::new(2., 2.)));
        canvas.set_blend_mode(BlendMode::ALPHA);

        // canvas.draw(&self.cards, DrawParam::new()
        //     .src(self.cards.uv_rect(self.x,self.y,self.w,self.h))
        //     .dest(Vec2::new(10.0,10.0)));
        //
        // let text = graphics::Text::new(TextFragment::new(format!("x={} y={} w={} h={}",
        //                                                          self.x, self.y, self.w, self.h)));
        // canvas.draw(&text, DrawParam::new().dest(Vec2::new(500.0,500.0)));

        self.get_scene().borrow_mut().draw(ctx, &mut canvas);
        canvas.finish(ctx)
    }
}
