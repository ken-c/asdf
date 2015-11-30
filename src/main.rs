
// based on http://piston-tutorial.logdown.com/pages/table-of-contents


extern crate piston_window;

use piston_window::*;

mod object;
use object::Object;


struct Game {
    player: Object,
    up: bool, down: bool, left: bool, right: bool
}

impl Game {
    fn new() -> Game {
        Game {
            player: Object::new(),
            up: false, down: false, left: false, right: false
        }
    }
    fn update(&mut self, upd: UpdateArgs) {
        if self.left { self.player.rot(-90.0 * upd.dt); }
        if self.right { self.player.rot(90.0 * upd.dt); }
        if self.up {
            self.player.forward(100.0*upd.dt);
        }
        if self.down {
            self.player.forward(-100.0*upd.dt);
        }
    }
    fn render(&mut self, ren: RenderArgs, e: PistonWindow) {
        e.draw_2d(|c, g| {
            // background to black
            clear([0.0; 4], g);

            let center = c.transform.trans(
                (ren.width/2) as f64, (ren.height/2) as f64);
            self.player.render(g, center);
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
