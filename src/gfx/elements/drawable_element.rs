use ggez::Context;
use ggez::graphics::{Canvas, Drawable, DrawParam, GraphicsContext, Image, Rect, Text};
use ggez::mint::Point2;

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

    fn passive_toggle() -> DrawableElementState {
        Self::new(DrawableElementState::TOGGLED)
    }

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

    pub fn hidden(mut self) -> Self {
        self.element_visibility = DrawableElementVisibility::Hidden;
        self
    }

    pub fn hide(&mut self) {
        self.element_visibility = DrawableElementVisibility::Hidden;
    }

    pub fn visible(mut self) -> Self {
        self.element_visibility = DrawableElementVisibility::Visible;
        self
    }

    pub fn show(&mut self) {
        self.element_visibility = DrawableElementVisibility::Visible;
    }
}
