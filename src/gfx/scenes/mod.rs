use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::Rc;

use ggez::Context;
use ggez::graphics::Canvas;

use crate::GameContext;
use crate::gfx::scenes::play_round::PlayRoundScene;
use crate::gfx::scenes::end_of_game::EndOfGameScene;
use crate::gfx::scenes::end_of_round::EndOfRoundScene;
use crate::gfx::scenes::number_of_players::NumberOfPlayersScene;

pub mod number_of_players;
mod play_round;
mod end_of_round;
mod end_of_game;

pub trait Scene {
    fn update(&mut self, game_ctx: &mut GameContext, ctx: &mut Context);
    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas);
}

pub struct Scenes {
    pub map: HashMap<SceneType, Rc<RefCell<dyn Scene>>>,
}

#[derive(Hash, Eq, PartialEq)]
#[allow(unused)]
pub enum SceneType {
    ChooseNumberOfPlayers,
    PlayRound,
    RoundSummary,
    EndOfRound,
    EndOfGame,
}

impl Scenes {

    pub fn from_context(ctx: &Context) -> Self {
        //TODO: get rid of this hashmap
        let mut map: HashMap<SceneType, Rc<RefCell<dyn Scene>>> = HashMap::new();

        map.insert(
            SceneType::ChooseNumberOfPlayers,
            Rc::new(RefCell::new(NumberOfPlayersScene::new(ctx))),
        );

        map.insert(
            SceneType::PlayRound,
            Rc::new(RefCell::new(PlayRoundScene::new(ctx))),
        );

        map.insert(
            SceneType::EndOfRound,
            Rc::new(RefCell::new(EndOfRoundScene::new(ctx))),
        );

        map.insert(
            SceneType::EndOfGame,
            Rc::new(RefCell::new(EndOfGameScene::new(ctx))),
        );

        Self { map }
    }

    pub fn get(&self, scene_type: SceneType) -> Rc<RefCell<dyn Scene>> {
        self.map.get(&scene_type).unwrap().clone()
    }
}
