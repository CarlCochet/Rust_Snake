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

static SIZE_WINDOW: u32 = 800;
static SIZE_SQUARE: u32 = 50;
static SIZE_GRID: u32 = SIZE_WINDOW / SIZE_SQUARE;

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
    score: u32,
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

        let mut k = 0;
        for i in 0..self.score {
            
            let square = graphics::rectangle::square(
                ((i % SIZE_GRID) * SIZE_SQUARE) as f64, 
                (k * SIZE_SQUARE) as f64, 
                SIZE_SQUARE as f64);

            if (i % SIZE_GRID) > SIZE_GRID - 2 {
                k += 1;
            }

            self.gl.draw(arg.viewport(), |c, gl| {
                let transform = c.transform;

                graphics::rectangle([0.0, 0.0, 0.0, 0.1], square, transform, gl);
            })
        }
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
        if self.snake.refresh == true {
            self.snake.refresh = false;
            self.score = 0;
        }
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
    refresh:bool,
}
impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;
        
        let BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body.iter().map(|&(x, y)| {
            graphics::rectangle::square(
                (x * SIZE_SQUARE as i32) as f64, 
                (y * SIZE_SQUARE as i32) as f64, 
                SIZE_SQUARE as f64)
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

        if new_head.0 > SIZE_GRID as i32 - 1 {
            new_head.0 = 0;
        } else if new_head.0 < 0 {
            new_head.0 = SIZE_GRID as i32 - 1;
        }
        if new_head.1 > SIZE_GRID as i32 - 1 {
            new_head.1 = 0;
        } else if new_head.1 < 0 {
            new_head.1 = SIZE_GRID as i32 - 1;
        }

        let snake_clone = self.body.clone();
        for block in snake_clone.iter() {
            if block.0 == new_head.0 && block.1 == new_head.1 {
                if block.0 != self.body.back().expect("No back found.").0 || block.1 != self.body.back().expect("No back found.").1 {
                    self.body = LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter());
                    self.dir = Direction::Right;
                    self.grow = false;
                    new_head = (*self.body.front().expect("Snake has no body.")).clone();
                    self.refresh = true;
                }
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
    pos: (i32, i32),
    color: [f32; 4],
}
impl Fruit {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        let square = graphics::rectangle::square(
            (self.pos.0 * SIZE_SQUARE as i32) as f64, 
            (self.pos.1 * SIZE_SQUARE as i32) as f64, 
            SIZE_SQUARE as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(self.color, square, transform, gl);
        })
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();
        self.pos.0 = rng.gen_range(0, SIZE_GRID as i32);
        self.pos.1 = rng.gen_range(0, SIZE_GRID as i32);
        self.color = [rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), 1.0];
    }
}


fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
            "Snake Game",
            [SIZE_WINDOW, SIZE_WINDOW]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()), dir: Direction::Right, grow: false, refresh: false},
        fruit: Fruit {pos: (SIZE_GRID as i32 / 2,  SIZE_GRID as i32 / 2), color: [1.0, 0.0, 0.0, 1.0]},
        lastKey: Key::Right,
        score: 0,
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
            game.score += 1;
            println!("{}", game.score);
        }
    }
}
