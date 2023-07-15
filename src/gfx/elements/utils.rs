use ggez::input::mouse;
use ggez::input::mouse::CursorIcon;
use ggez::Context;

use crate::gfx::elements::drawable_element::{DrawableElement, DrawableElementState};

pub(crate) fn handle_hover_gfx(
    ctx: &mut Context,
    element: &mut DrawableElement,
) {
    if element.check_hovered(ctx) {
        mouse::set_cursor_type(ctx, CursorIcon::Hand);
        element.element_state = DrawableElementState::new(DrawableElementState::HOVERED)
    } else {
        element.element_state = DrawableElementState::default()
    }
}
