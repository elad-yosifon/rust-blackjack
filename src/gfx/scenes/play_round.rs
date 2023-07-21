use std::ops::{Add, AddAssign, Mul, Sub};

use ggez::Context;
use ggez::graphics::{Canvas, DrawParam};
use ggez::graphics::{Color, PxScale};
use ggez::graphics::Text;
use ggez::graphics::TextFragment;
use ggez::input::mouse;
use ggez::input::mouse::CursorIcon;
use ggez::mint::Point2;

use crate::{at, GameContext};
use crate::cards::card::{CardSymbol, Suit};
use crate::gameplay::actor::ActorRole;
use crate::gameplay::hand::{Hand, HandState};
use crate::gfx::elements::cards_sprite::CardsSprite;
use crate::gfx::elements::drawable_element::DrawableElement;
use crate::gfx::elements::utils::handle_hover_gfx;
use crate::gfx::scenes::{Scene, SceneType};

pub struct PlayRoundScene {
    cards_sprite: CardsSprite,
    players: Vec<DrawableElement>,
    dealer: DrawableElement,
    hit_btn: DrawableElement,
    stay_btn: DrawableElement,
    split_btn: DrawableElement,
    cards_layout: Vec<(CardSymbol, Suit, f32, f32)>,
}

impl PlayRoundScene {
    pub fn new(ctx: &Context) -> Self {
        let (_, w) = ctx.gfx.size();

        let mut d = Text::new(TextFragment::new("Dealer"));
        d.set_scale(PxScale::from(40.0));

        let mut p1 = Text::new(TextFragment::new("p1"));
        p1.set_scale(PxScale::from(60.0));

        let mut p2 = Text::new(TextFragment::new("p2"));
        p2.set_scale(PxScale::from(60.0));

        let mut p3 = Text::new(TextFragment::new("p3"));
        p3.set_scale(PxScale::from(60.0));

        let mut hit_btn = Text::new(TextFragment::new("HIT"));
        hit_btn.set_scale(PxScale::from(60.0));

        let mut stay_btn = Text::new(TextFragment::new("STAY"));
        stay_btn.set_scale(PxScale::from(60.0));

        let mut split_btn = Text::new(TextFragment::new("SPLIT"));
        split_btn.set_scale(PxScale::from(60.0));

        Self::_new(
            CardsSprite::new(ctx),
            vec![
                DrawableElement::new_text(p1, Point2::from([700., 600.])),
                DrawableElement::new_text(p2, Point2::from([400., 650.])).hidden(),
                DrawableElement::new_text(p3, Point2::from([100., 600.])).hidden(),
            ],
            DrawableElement::new_text(d, Point2::from([w / 2.0, 50.])),
            DrawableElement::new_text_button(hit_btn, Point2::from([700., 50.])),
            DrawableElement::new_text_button(stay_btn, Point2::from([700., 100.])),
            DrawableElement::new_text_button(split_btn, Point2::from([700., 150.])),
        )
    }

    fn _new(
        cards_sprite: CardsSprite,
        players: Vec<DrawableElement>,
        dealer: DrawableElement,
        hit_btn: DrawableElement,
        stay_btn: DrawableElement,
        split_btn: DrawableElement,
    ) -> Self {
        Self {
            cards_sprite,
            players,
            dealer,
            hit_btn,
            stay_btn,
            split_btn,
            cards_layout: vec![],
        }
    }
}

impl Scene for PlayRoundScene {
    fn update(&mut self, game_ctx: &mut GameContext, ctx: &mut Context) {
        mouse::set_cursor_type(ctx, CursorIcon::Default);

        let n = game_ctx.game.number_of_players;

        self.players.iter_mut().enumerate().for_each(|(i, p)| {
            if i < n {
                p.show();
            } else {
                p.hide();
            }
        });

        let round = &mut game_ctx.game.current_round;

        if round.actor_cursor >= round.actors.len() {
            game_ctx.current_scene = SceneType::RoundSummary;
            return;
        }

        let actor = at!(mut round.actors, round.actor_cursor);
        let dealer_turn = matches!(actor.role, ActorRole::Dealer);

        if round.hand_cursor >= actor.hands.len() {
            round.actor_cursor.add_assign(1);
            round.hand_cursor = 0;
            return;
        }

        let hand = actor.hand_at_mut(round.hand_cursor);

        // show/hide buttons
        if dealer_turn {
            self.hit_btn.hide();
            self.stay_btn.hide();
            self.split_btn.hide();
        } else {
            self.hit_btn.show();
            self.stay_btn.show();
            if hand.is_splitable() {
                self.split_btn.show();
            } else {
                self.split_btn.hide();
            }
        }

        handle_hover_gfx(ctx, &mut self.hit_btn);
        handle_hover_gfx(ctx, &mut self.stay_btn);
        handle_hover_gfx(ctx, &mut self.split_btn);

        match hand.state {
            HandState::Undefined => {
                if self.hit_btn.check_clicked(ctx) {
                    println!("Hand --> HIT \n");
                    hand.deal_card(round.deck.draw_card());
                    round.update();
                } else if self.stay_btn.check_clicked(ctx) {
                    println!("Hand --> STAY \n");
                    hand.state = HandState::Finished;
                    round.hand_cursor.add_assign(1);
                } else if self.split_btn.check_clicked(ctx) {
                    println!("Hand --> SPLIT \n");
                    let new_hand = hand.split(round.deck.draw_card(), round.deck.draw_card());
                    actor.hands.insert(round.hand_cursor + 1, new_hand);
                    round.update();
                }
            }
            HandState::Finished | HandState::Bust | HandState::Blackjack => {
                round.hand_cursor.add_assign(1);
            }
        }

        let mut cards_layout: Vec<(CardSymbol, Suit, f32, f32)> = vec![];
        game_ctx
            .game
            .current_round
            .actors
            .iter()
            .enumerate()
            .for_each(|(actor_i, actor)| match actor.role {
                ActorRole::Player => {
                    actor.hands.iter().enumerate().for_each(|(hand_i, hand)| {
                        let (anchor_x, anchor_y) = calculate_player_hand_location(actor_i, hand_i);
                        push_card_layout(&mut cards_layout, hand, anchor_x, anchor_y);
                    });
                }
                ActorRole::Dealer => {
                    let (anchor_x, anchor_y) = (300., 100.);
                    push_card_layout(&mut cards_layout, actor.hand_at(0), anchor_x, anchor_y);
                }
            });
        self.cards_layout = cards_layout;
    }

    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) {
        self.dealer
            .draw(ctx, canvas, DrawParam::new().color(Color::WHITE));

        let elements = &self.players;
        elements.iter().filter(|de| de.is_visible()).for_each(|de| {
            de.draw(ctx, canvas, DrawParam::new().color(Color::WHITE));
        });

        render_de(ctx, canvas, &self.hit_btn);
        render_de(ctx, canvas, &self.stay_btn);
        render_de(ctx, canvas, &self.split_btn);

        self.cards_layout.iter().for_each(|(symbol, suit, x, y)| {
            self.cards_sprite
                .draw_card_at_point(canvas, symbol, suit, x, y);
        })
    }
}

fn render_de(ctx: &mut Context, canvas: &mut Canvas, element: &DrawableElement) {
    if element.is_visible() {
        if element.is_hovered() {
            element.draw(ctx, canvas, DrawParam::new().color(Color::YELLOW));
        } else {
            element.draw(ctx, canvas, DrawParam::new().color(Color::WHITE));
        }
    }
}

const CARD_PADDING_RIGHT: f32 = 20.0;

fn push_card_layout(
    mut cards_layout: &mut Vec<(CardSymbol, Suit, f32, f32)>,
    hand: &Hand,
    anchor_x: f32,
    anchor_y: f32,
) {
    hand.cards
        .iter()
        .enumerate()
        .map(|(i, card)| {
            let x = CARD_PADDING_RIGHT.mul(i.add(1) as f32).add(anchor_x);
            (card.value, card.suit, x, anchor_y)
        })
        .for_each(|card_layout| cards_layout.push(card_layout));
}

fn calculate_player_hand_location(actor_i: usize, hand_i: usize) -> (f32, f32) {
    let distance_to_left = (300.).mul(actor_i as f32);
    let distance_to_top = (200.).mul(hand_i as f32);

    let actor_y = match actor_i {
        0 => 400_f32,
        1 => 500_f32,
        2 => 400_f32,
        _ => unimplemented!("")
    };
    let (anchor_x, anchor_y) = (
        600_f32.sub(distance_to_left),
        actor_y.sub(distance_to_top));
    (anchor_x, anchor_y)
}
