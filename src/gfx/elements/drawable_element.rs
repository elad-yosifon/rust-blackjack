use ggez::event::MouseButton;
use ggez::graphics::{Canvas, DrawParam, Drawable, GraphicsContext, Image, Rect, Text};
use ggez::mint::Point2;
use ggez::Context;

pub enum DrawableElementVisibility {
    Hidden,
    Visible,
}

pub struct DrawableElementState {
    bitmap: u8,
}

impl DrawableElementState {
    pub fn default() -> Self {
        Self {
            bitmap: Self::DEFAULT,
        }
    }

    pub fn new(bitmap: u8) -> Self {
        Self { bitmap }
    }

    fn button() -> DrawableElementState {
        Self::new(DrawableElementState::HOVERED | DrawableElementState::PRESSED)
    }

    #[allow(unused)]
    fn passive_toggle() -> DrawableElementState {
        Self::new(DrawableElementState::TOGGLED)
    }

    #[allow(unused)]
    fn active_toggle() -> DrawableElementState {
        Self::new(
            DrawableElementState::HOVERED
                | DrawableElementState::PRESSED
                | DrawableElementState::TOGGLED,
        )
    }

    pub const DEFAULT: u8 = 0b000;
    pub const HOVERED: u8 = 0b001;
    pub const PRESSED: u8 = 0b010;
    pub const TOGGLED: u8 = 0b100;

    pub(crate) fn has(&self, bitmap: u8) -> bool {
        self.bitmap & bitmap > 0
    }
}

#[allow(unused)]
pub enum DrawableElementType {
    Text,
    Image,
    TextButton,
    ImageButton,
}

pub struct DrawableElement {
    pub element_type: DrawableElementType,
    pub element_state: DrawableElementState,
    pub possible_states: DrawableElementState,
    pub element_visibility: DrawableElementVisibility,

    pub position: Point2<f32>,

    pub text: Option<Text>,
    pub image: Option<Image>,
}

impl DrawableElement {
    pub fn dimensions(&self, gfx: &GraphicsContext) -> Option<Rect> {
        let rect = match &self.element_type {
            DrawableElementType::Text | DrawableElementType::TextButton => {
                self.text.as_ref().unwrap().dimensions(gfx)
            }
            DrawableElementType::Image | DrawableElementType::ImageButton => {
                self.image.as_ref().unwrap().dimensions(gfx)
            }
        };

        if let Some(d) = rect {
            let x = self.position.x - (d.w / 2.0);
            let y = self.position.y - (d.h / 2.0);
            return Some(Rect::new(x, y, d.w, d.h));
        }
        None
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, draw_param: DrawParam) {
        if let Some(rect) = &self.dimensions(&ctx.gfx) {
            match &self.element_type {
                DrawableElementType::Text | DrawableElementType::TextButton => {
                    canvas.draw(self.text.as_ref().unwrap(), draw_param.dest(rect.point()));
                }
                DrawableElementType::Image | DrawableElementType::ImageButton => {
                    canvas.draw(self.image.as_ref().unwrap(), draw_param.dest(rect.point()));
                }
            }
        }
    }
}

impl DrawableElement {
    pub fn new_text(text: Text, position: Point2<f32>) -> DrawableElement {
        Self {
            position,
            image: None,
            text: Some(text),
            element_type: DrawableElementType::Text,
            element_state: DrawableElementState::default(),
            possible_states: DrawableElementState::default(),
            element_visibility: DrawableElementVisibility::Visible,
        }
    }

    pub fn new_text_button(text: Text, position: Point2<f32>) -> DrawableElement {
        Self {
            position,
            image: None,
            text: Some(text),
            element_type: DrawableElementType::TextButton,
            element_state: DrawableElementState::default(),
            possible_states: DrawableElementState::button(),
            element_visibility: DrawableElementVisibility::Visible,
        }
    }

    fn mouse_is_over(element: &DrawableElement, gfx: &GraphicsContext, mouse_pos: Point2<f32>) -> bool {
        if let Some(r) = element.dimensions(gfx) {
            if r.contains(mouse_pos){
                return true;
            }
        }
        false
    }

    #[allow(unused)]
    pub fn check_clicked(&self, ctx: &Context) -> bool {
        //TODO: be nice and support "click-zone" states

        self.is_visible()
            && self.is_clickable()
            && ctx.mouse.button_pressed(MouseButton::Left)
            && Self::mouse_is_over(self, &ctx.gfx, ctx.mouse.position())
    }

    #[allow(unused)]
    pub fn check_hovered(&self, ctx: &Context) -> bool {
        self.is_visible()
            && self.is_hoverable()
            && Self::mouse_is_over(self, &ctx.gfx, ctx.mouse.position())
    }


    #[allow(unused)]
    pub fn is_hovered(&self) -> bool {
        self.element_state.has(DrawableElementState::HOVERED)
    }

    #[allow(unused)]
    pub fn is_pressed(&self) -> bool {
        self.element_state.has(DrawableElementState::PRESSED)
    }

    #[allow(unused)]
    pub fn is_toggled(&self) -> bool {
        self.element_state.has(DrawableElementState::TOGGLED)
    }

    #[allow(unused)]
    pub fn is_hoverable(&self) -> bool {
        self.possible_states.has(DrawableElementState::HOVERED)
    }

    #[allow(unused)]
    pub fn is_clickable(&self) -> bool {
        self.possible_states.has(DrawableElementState::PRESSED)
    }

    #[allow(unused)]
    pub fn is_toggleable(&self) -> bool {
        self.possible_states.has(DrawableElementState::TOGGLED)
    }

    #[allow(unused)]
    pub fn is_hidden(&self) -> bool {
        matches!(self.element_visibility, DrawableElementVisibility::Hidden)
    }

    #[allow(unused)]
    pub fn is_visible(&self) -> bool {
        matches!(self.element_visibility, DrawableElementVisibility::Visible)
    }

    #[allow(unused)]
    pub fn hidden(mut self) -> Self {
        self.element_visibility = DrawableElementVisibility::Hidden;
        self
    }

    #[allow(unused)]
    pub fn hide(&mut self) {
        self.element_visibility = DrawableElementVisibility::Hidden;
    }

    #[allow(unused)]
    pub fn visible(mut self) -> Self {
        self.element_visibility = DrawableElementVisibility::Visible;
        self
    }

    #[allow(unused)]
    pub fn show(&mut self) {
        self.element_visibility = DrawableElementVisibility::Visible;
    }
}
