use std::iter::Filter;
use std::slice::Iter;

use ggez::Context;
use ggez::input::mouse;
use ggez::input::mouse::CursorIcon;

use crate::GameContext;
use crate::gfx::elements::drawable_element::{DrawableElement, DrawableElementState, DrawableElementVisibility};

pub(crate) fn handle_hovers(
    game_ctx: &mut GameContext,
    ctx: &mut Context,
    elements: &mut Vec<DrawableElement>,
) {
    let mouse_pos = ctx.mouse.position();
    for (i, element) in elements.iter_mut().enumerate() {
        let possible_states = &element.possible_states;
        if possible_states.has(DrawableElementState::PRESSED)
            || possible_states.has(DrawableElementState::HOVERED)
        {
            if let Some(r) = element.dimensions(&ctx.gfx) {
                if r.contains(mouse_pos) {
                    mouse::set_cursor_type(ctx, CursorIcon::Hand);
                    element.element_state =
                        DrawableElementState::new(DrawableElementState::HOVERED);
                } else {
                    element.element_state = DrawableElementState::default();
                }
            }
        }
    }
}


pub(crate) fn iter_visible<'a>(elements: &'a Vec<DrawableElement>) -> Filter<Iter<'a, DrawableElement>, fn(&&'a DrawableElement) -> bool> {
    Iter::filter(elements.iter(), |de| matches!(de.element_visibility, DrawableElementVisibility::Visible))
}