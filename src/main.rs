
// based on http://piston-tutorial.logdown.com/pages/table-of-contents


extern crate piston_window;

use piston_window::*;


struct Game {
    rot: f64,
    x: f64,
    y: f64,
    up: bool, down: bool, left: bool, right: bool
}

impl Game {
    fn new() -> Game {
        Game {
            rot: 0.0, x: 0.0, y: 0.0,
            up: false, down: false, left: false, right: false
        }
    }
    fn update(&mut self, upd: UpdateArgs) {
        self.rot += 90.0 * upd.dt;
        if self.up { self.y += -50.0 * upd.dt; }
        if self.down { self.y += 50.0 * upd.dt; }
        if self.left { self.x += -50.0 * upd.dt; }
        if self.right { self.x += 50.0 * upd.dt; }
    }
    fn render(&mut self, ren: RenderArgs, e: PistonWindow) {
        e.draw_2d(|c, g| {
            // background to black
            clear([0.0; 4], g);
            // draw red triangle centered about 0,0
            let red = [1.0, 0.0, 0.0, 1.0];
            let coords = &[ [0.0, 50.0], [25.0, -50.0], [-25.0, -50.0] ];
            let center = c.transform.trans(self.x, self.y)
                .rot_deg(self.rot);
            polygon(red, coords, center, g);
        });
    }
    fn input(&mut self, inp: Input) {
        match inp {
            Input::Press(b) => {
                match b {
                    Button::Keyboard(Key::Up) => { self.up = true; }
                    Button::Keyboard(Key::Down) => { self.down = true; }
                    Button::Keyboard(Key::Left) => { self.left = true; }
                    Button::Keyboard(Key::Right) => { self.right = true; }
                    _ => { }
                }
            }
            Input::Release(b) => {
                match b {
                    Button::Keyboard(Key::Up) => { self.up = false; }
                    Button::Keyboard(Key::Down) => { self.down = false; }
                    Button::Keyboard(Key::Left) => { self.left = false; }
                    Button::Keyboard(Key::Right) => { self.right = false; }
                    _ => { }
                }
            }
            _ => { }
        }
    }
}


fn main() {
    println!("asdf");

    let window: PistonWindow =
        WindowSettings::new("Hello Piston!", [600, 600])
        .exit_on_esc(true).build().unwrap();

    let mut game = Game::new();
    for e in window {
        match e.event {
            Some(Event::Update(upd)) => {
                game.update(upd);
            }
            Some(Event::Render(ren)) => {
                game.render(ren, e);
            }
            Some(Event::Input(inp)) => {
                game.input(inp);
            }
            _ => { }
        }
    }    

}
