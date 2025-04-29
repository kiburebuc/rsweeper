use macroquad::{
    shapes::*,
    color::{Color, colors::*},
    text::{draw_text, get_text_center},
    math::Vec2,
};

pub const BASE: Color = Color::from_hex(0xc0c0c0);
pub const GRID: Color = Color::from_hex(0x808080);
pub const FLAG_COLOR: Color = Color::from_hex(0xfe0000);

pub const RAISED: [Color; 3] = [WHITE, BASE, GRID];
pub const RECESSED: [Color; 3] = [GRID, BASE, WHITE];

#[derive(Default, Clone, Copy)]
pub enum Mouse {
    #[default]
    None,
    Drag(Vec2, bool),
    Released(Vec2, bool),
}

#[derive(Default)]
pub struct Widget {
    pub screen: Vec2, // size of virtual screen
    pub offset: Vec2, // offset to draw from origin
    pub mouse: Mouse,
}

pub trait WidgetImpl {
    fn widget(&self) -> &Widget;
    fn widget_mut(&mut self) -> &mut Widget;
    fn draw(&mut self);
    fn update(&mut self);


    fn screen_mut(&mut self) -> &mut Vec2 { &mut self.widget_mut().screen }
    fn screen(&self) -> Vec2 { self.widget().screen }
    fn offset_mut(&mut self) -> &mut Vec2 { &mut self.widget_mut().offset }
    fn offset(&self) -> Vec2 { self.widget().offset }
    fn mouse_mut(&mut self) -> &mut Mouse { &mut self.widget_mut().mouse }
    fn mouse(&self) -> Mouse { self.widget().mouse }

    fn woff(&self, p: Vec2) -> Vec2 { self.offset() + p }

    fn draw_rect(&self, pos: Vec2, size: Vec2, color: Color) {
        let pos = self.woff(pos);
        draw_rectangle(pos.x, pos.y, size.x, size.y, color); 
    }
    
    fn draw_line(&self, pos: Vec2, end: Vec2, stroke: f32, color: Color) {
        let pos = self.woff(pos);
        let end = self.woff(end);
        draw_line(pos.x, pos.y, end.x, end.y, stroke, color); 
    }
    
    fn draw_tri(&self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color) {
        let p1 = self.woff(p1);
        let p2 = self.woff(p2);
        let p3 = self.woff(p3);
        draw_triangle(p1.into(), p2.into(), p3.into(), color);
    }

    fn draw_text(&self, t: &str, pos: Vec2, s: u16, c: Color) {
        let pos = self.woff(pos);
        draw_text(&t, pos.x, pos.y, s as f32, c);
    }
    fn draw_ctext(&self, t: &str, pos: Vec2, s: u16, c: Color) {
        let center = get_text_center(&t, None, s, 1., 0.);
        let pos = self.woff(pos) - center;
        draw_text(&t, pos.x, pos.y, s as f32, c); 
    }

    //    _________         _________ 
    //   |        /|       |  _____ /|
    //   |       / |  -->  | |     | |
    //   |      /  |  -->  | |     | |  
    //   |_____/___|  -->  | |     | |
    //   |   /     |  -->  | |     | |
    //   |  /      |  -->  | |     | |
    //   | /       |       | |_____| |
    //   |/_______ |       |/________|
    //
    //                         x y        w h
    fn draw_raised_rect(&self, pos: Vec2, size: Vec2, m: f32, c: [Color; 3]) {
        // shadow
        let half = size.y / 2.;
        self.draw_rect(pos, size, c[0]);
        self.draw_rect(pos.add_y(half), size.with_y(half), c[2]);

        self.draw_tri(pos, pos.add_y(size.y), pos + Vec2::splat(half), c[0]);
        self.draw_tri(pos.add_x(size.x), pos + size, 
            pos.add_x(size.x - half).add_y(half), c[2]);
        // base
        self.draw_rect(pos + Vec2::splat(m), size - Vec2::splat(m * 2.), c[1]);
    }

    fn local_mouse(&self, m: Vec2) -> Vec2 { m - self.offset() }
    fn in_rect(&self, m: Vec2, pos: Vec2, size: Vec2) -> bool {
        let m = self.local_mouse(m);
        (pos.x..size.x).contains(&m.x) && (pos.y..size.y).contains(&m.y) 
    }
}

pub trait ExtraVec2 {
    fn add_x(&self, x: f32) -> Self;
    fn add_y(&self, x: f32) -> Self;
}

impl ExtraVec2 for Vec2 {
    fn add_x(&self, x: f32) -> Self {
        Vec2 { x: self.x + x, y: self.y }
    }
    fn add_y(&self, y: f32) -> Self {
        Vec2 { x: self.x, y: self.y + y }
    }
}
