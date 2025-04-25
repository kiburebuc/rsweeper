use macroquad::{
    shapes::*,
    color::Color,
    text::{draw_text, get_text_center},
};

pub type Vec2 = (f32, f32);

#[derive(Default)]
pub struct Widget {
    pub screen: Vec2, // size of virtual screen
    pub offset: Vec2, // offset to draw from origin
    pub mouse: Option<(Vec2, bool)>,
    pub update: bool,
}

pub trait WidgetImpl {
    fn draw(&mut self);
    fn update(&mut self, m: Option<(Vec2, bool)>);
    fn screen(&self) -> Vec2;
    fn offset(&self) -> Vec2;

    fn woff(&self, p: Vec2) -> Vec2 { 
        (p.0 + self.offset().0, p.1 + self.offset().1)
    }

    fn draw_rect(&self, pos: Vec2, size: Vec2, color: Color) {
        let pos = self.woff(pos);
        draw_rectangle(pos.0, pos.1, size.0, size.1, color); 
    }
    
    fn draw_line(&self, pos: Vec2, end: Vec2, stroke: f32, color: Color) {
        let pos = self.woff(pos);
        let end = self.woff(end);
        draw_line(pos.0, pos.1, end.0, end.1, stroke, color); 
    }
    
    fn draw_tri(&self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color) {
        let p1 = self.woff(p1);
        let p2 = self.woff(p2);
        let p3 = self.woff(p3);
        draw_triangle(p1.into(), p2.into(), p3.into(), color);
    }

    fn draw_ctext(&self, t: &str, pos: Vec2, s: u16, c: Color) {
        let center = get_text_center(&t, None, s, 1., 0.);
        let pos = self.woff(pos);
        draw_text(&t, pos.0 - center.x, pos.1 - center.y, s as f32, c); 
    }

    //    _________         _________ 
    //   |   /    /|       |  _____ /|
    //   |  /____/ |  -->  | |     | |
    //   | /    /  |       | |_____| |
    //   |/____/___|       |/________|
    //
    //                         x y        w h
    fn draw_raised_rect(&self, pos: Vec2, size: Vec2, m: f32, c: [Color; 3]) {
        // shadow
        self.draw_rect(pos, size, c[0]);
        self.draw_rect((pos.0, pos.1 + size.1 / 2.), (size.0, size.1 / 2.), c[2]);

        self.draw_tri(pos, (pos.0 + size.1, pos.1), (pos.0, pos.1 + size.1), c[0]);
        self.draw_tri((pos.0 + size.0, pos.1), (pos.0 + size.0, pos.1 + size.1), 
            (pos.0 + size.0 - size.1, pos.1 + size.1), c[2]);
        // base
        self.draw_rect((pos.0 + m, pos.1 + m), (size.0 - m * 2., size.1 - m * 2.), c[1]);
    }

    fn local_mouse(&self, m: Vec2) -> Vec2 { (m.0 - self.offset().0, m.1 - self.offset().1) }
}
