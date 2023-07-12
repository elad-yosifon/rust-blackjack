use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use ggez::graphics::Canvas;
use ggez::Context;

use crate::gfx::scenes::number_of_players::NumberOfPlayersScene;
use crate::MyGame;

pub mod number_of_players;

pub trait Scene {
    fn update(&mut self, game: &mut MyGame, ctx: &mut Context);
    fn draw(&self, canvas: &mut Canvas);
}

pub struct Scenes {
    pub map: HashMap<SceneType, Rc<RefCell<dyn Scene>>>,
}

#[derive(Hash, Eq, PartialEq)]
pub enum SceneType {
    ChooseNumberOfPlayers,
    DealCards,
    PlayerTurn,
    DealerTurn,
    RoundSummary,
    ChooseEndOfRoundAction,
}

impl Scenes {
    pub fn default() -> Self {
        Self {
            map: HashMap::default(),
        }
    }

    pub fn from_game(game: &MyGame) -> Self {
        let mut map: HashMap<SceneType, Rc<RefCell<dyn Scene>>> = HashMap::new();

        map.insert(
            SceneType::ChooseNumberOfPlayers,
            Rc::new(RefCell::new(NumberOfPlayersScene::new(game))),
        );

        Self { map }
    }

    pub fn get(&self, scene_type: SceneType) -> Rc<RefCell<dyn Scene>> {
        self.map.get(&scene_type).unwrap().clone()
    }
}
