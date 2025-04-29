use crate::widget::*;
use macroquad::{
    color::colors::*,
    math::Vec2,
};

const SCALE: f32 = 2.;

const MARGIN: f32 = SCALE * 1.;
const MARGIN2: f32 = MARGIN * 2.;
const MARGIN3: f32 = MARGIN * 3.;
const MARGIN4: f32 = MARGIN * 4.;
const MARGIN6: f32 = MARGIN * 6.;

const fn pix(n: f32) -> f32 { SCALE * n }

const TITLEHEIGHT: f32 = 18. * SCALE;
const MENUHEIGHT: f32 = 20. * SCALE;


#[derive(Default)]
pub struct Window<T: WidgetImpl> {
    pub widget: Widget,
    pub drag_off: Option<Vec2>,
    pub app: T,
}

impl<T: WidgetImpl> Window<T> {
    pub fn set_size(&mut self, size: Vec2) { self.widget.screen = size * SCALE }
}

impl<T: WidgetImpl> WidgetImpl for Window<T> {
    fn draw(&mut self) {
        // draw window
        let screen = self.screen();
        self.draw_raised_rect(Vec2::ZERO, screen, MARGIN2, RAISED);
        self.draw_rect(Vec2::splat(MARGIN3),
            screen.add_x(-MARGIN6).with_y(TITLEHEIGHT), DARKBLUE);
        self.draw_raised_rect(screen.add_x(-pix(56.)).with_y(pix(5.)), 
            (pix(16.), pix(14.)).into(), MARGIN, RAISED);
        self.draw_raised_rect(screen.add_x(-pix(40.)).with_y(pix(5.)), 
            (pix(16.), pix(14.)).into(), MARGIN, RAISED);
        self.draw_raised_rect(screen.add_x(-pix(22.)).with_y(pix(5.)), 
            (pix(16.), pix(14.)).into(), MARGIN, RAISED);
        
        self.draw_text("Game Help", Vec2::ZERO.add_y(54.) + pix(8.), 30, BLACK);
        self.app.draw();
    }

    fn update(&mut self) {
        if let Mouse::Drag(m, _) = self.widget.mouse {
            if let Some(o) = self.drag_off {
                self.widget.offset = m - o;
            } else {
                if self.in_rect(m, Vec2::splat(MARGIN2), 
                        self.screen().add_x(-MARGIN4).with_y(TITLEHEIGHT)) {
                    self.drag_off = Some(self.local_mouse(m));
                }
            }
        } else { self.drag_off = None; }
        
        *self.app.mouse_mut() = self.widget.mouse;
        *self.app.screen_mut() = self.screen().add_y(-TITLEHEIGHT-MENUHEIGHT) - MARGIN6;
        *self.app.offset_mut() = self.offset().add_y(MENUHEIGHT + TITLEHEIGHT) + MARGIN3;
        self.app.update();
    }
    fn widget(&self) -> &Widget { &self.widget }
    fn widget_mut(&mut self) -> &mut Widget { &mut self.widget }
}
