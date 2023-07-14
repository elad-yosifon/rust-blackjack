use std::iter::Filter;
use std::slice::Iter;

use ggez::input::mouse;
use ggez::input::mouse::CursorIcon;
use ggez::Context;

use crate::gfx::elements::drawable_element::{
    DrawableElement, DrawableElementState, DrawableElementVisibility,
};
use crate::GameContext;

pub(crate) fn handle_hovers(
    _game_ctx: &mut GameContext,
    ctx: &mut Context,
    elements: &mut [DrawableElement],
) {
    let mouse_pos = ctx.mouse.position();
    for element in elements.iter_mut() {
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

pub(crate) fn iter_visible(
    elements: &[DrawableElement],
) -> Filter<Iter<DrawableElement>, fn(&&DrawableElement) -> bool> {
    Iter::filter(elements.iter(), |de| {
        matches!(de.element_visibility, DrawableElementVisibility::Visible)
    })
}
