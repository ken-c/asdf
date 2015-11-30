
use piston_window::*;

use std::f64;


pub struct Object {
    x: f64,
    y: f64,
    theta: f64  // rotation in degrees
}


#[allow(dead_code)]

impl Object {
    pub fn new() -> Object {
        Object { x: 0.0, y: 0.0, theta: 0.0 }
    }

    pub fn mov(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    pub fn mov_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn rot(&mut self, dtheta: f64) {
        self.theta += dtheta;
    }

    pub fn rot_to(&mut self, theta: f64) {
        self.theta = theta;
    }

    pub fn forward(&mut self, delta: f64) {
        let angle = self.theta/180.0*f64::consts::PI;
        let f = angle.sin_cos();
        self.x += delta * f.1;
        self.y += delta * f.0;
    }

    // FIXME: color should be parameter
    pub fn render<G>(&mut self, g: &mut G, view: math::Matrix2d) where G: Graphics {
        // render a red triangle in standard position
        let red = [1.0, 0.0, 0.0, 1.0];
        //        let coords = &[ [0.0, -10.0], [-5.0, 10.0], [5.0, 10.0] ];
        let coords = &[ [10.0, 0.0], [-10.0, 5.0], [-10.0, -5.0] ];
        let center = view.trans(self.x, self.y).rot_deg(self.theta);
        polygon(red, coords, center, g);
    }
}

