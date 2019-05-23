extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };

use std::collections::LinkedList;
use std::iter::FromIterator;
use rand::Rng;

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    fruit: Fruit,
    lastKey: Key,
}
impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics;

        let WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(WHITE, gl);
        });

        self.snake.render(&mut self.gl, arg);
        self.fruit.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match self.lastKey {
            Key::Up
                if last_direction != Direction::Down => Direction::Up,
            Key::Down
                if last_direction != Direction::Up => Direction::Down,
            Key::Left 
                if last_direction != Direction::Right => Direction::Left,
            Key::Right
                if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        };

        self.snake.update();
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();
        
        self.lastKey = match btn {
            &Button::Keyboard(Key::Up) 
                if last_direction != Direction::Down => Key::Up,
            &Button::Keyboard(Key::Down) 
                if last_direction != Direction::Up => Key::Down,
            &Button::Keyboard(Key::Left) 
                if last_direction != Direction::Right => Key::Left,
            &Button::Keyboard(Key::Right) 
                if last_direction != Direction::Left => Key::Right,
            _ => self.lastKey
        };
    }

}

struct Snake {
    body: LinkedList<(i32, i32)>,
    dir: Direction,
    grow: bool,
}
impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;
        
        let BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body.iter().map(|&(x, y)| {
            graphics::rectangle::square(
                (x * 50) as f64, 
                (y * 50) as f64, 
                50_f64)
        })
        .collect();
        

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            
            squares.into_iter().for_each(|square| graphics::rectangle(BLACK, square, transform, gl));
        })
    }

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body.")).clone();

        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        if new_head.0 > 15 {
            new_head.0 = 0;
        } else if new_head.0 < 0 {
            new_head.0 = 15;
        }
        if new_head.1 > 15 {
            new_head.1 = 0;
        } else if new_head.1 < 0 {
            new_head.1 = 15;
        }

        let snake_clone = self.body.clone();
        for block in snake_clone.iter() {
            if block.0 == new_head.0 && block.1 == new_head.1 {
                self.body = LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter());
                self.dir = Direction::Right;
                self.grow = false;
            }
        }

        self.body.push_front(new_head);
        if !self.grow {
            self.body.pop_back().unwrap();
        } else {
            self.grow = false;
        }
    }
}

struct Fruit {
    pos: (i32, i32)
}
impl Fruit {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;
        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square(
            (self.pos.0 * 50) as f64, 
            (self.pos.1 * 50) as f64, 
            50_f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(RED, square, transform, gl);
        })
    }

    fn update(&mut self) {
        self.pos.0 = rand::thread_rng().gen_range(0, 16);
        self.pos.1 = rand::thread_rng().gen_range(0, 16);
    }
}


fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
            "Snake Game",
            [800, 800]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()), dir: Direction::Right, grow: false},
        fruit: Fruit {pos: (7, 7)},
        lastKey: Key::Right,
    };
    
    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }

        if game.snake.body.front().expect("No 0 index.").0 == game.fruit.pos.0 && game.snake.body.front().expect("No 1 index.").1 == game.fruit.pos.1 {
            game.fruit.update();
            game.snake.grow = true;
        }
    }
}
