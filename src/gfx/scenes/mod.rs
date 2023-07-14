use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::Rc;

use ggez::Context;
use ggez::graphics::Canvas;

use crate::GameContext;
use crate::gfx::scenes::deal_cards::DealCardsScene;
use crate::gfx::scenes::number_of_players::NumberOfPlayersScene;

pub mod number_of_players;
mod deal_cards;

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

    pub fn from_game(game_ctx: &GameContext) -> Self {
        let mut map: HashMap<SceneType, Rc<RefCell<dyn Scene>>> = HashMap::new();

        map.insert(
            SceneType::ChooseNumberOfPlayers,
            Rc::new(RefCell::new(NumberOfPlayersScene::new(game_ctx))),
        );

        map.insert(
            SceneType::DealCards,
            Rc::new(RefCell::new(DealCardsScene::new(game_ctx))),
        );

        Self { map }
    }

    pub fn get(&self, scene_type: SceneType) -> Rc<RefCell<dyn Scene>> {
        self.map.get(&scene_type).unwrap().clone()
    }
}
