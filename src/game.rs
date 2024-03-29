use rand::{thread_rng, Rng};
use piston_window::{Context, G2d, Key};
use piston_window::types::Color;
use crate::snake::{Snake, Direction};
use crate::drawing::{draw_rectangle, draw_block};

const BORDER_COLOR: Color = [0.741, 0.765, 0.78, 1.0];
const FOOD_COLOR: Color = [0.5, 0.5, 0.5, 1.0];


pub struct Game {
    snake: Snake,
    width: i32,
    height: i32,
    pub game_over: bool,
    food_exist: bool,
    food_x: i32,
    food_y: i32
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
	Game {
	    snake: Snake::new(2, 2),
	    width: width,
	    height: height,
	    game_over: false,
            food_exist: true,
            food_x: 5,
            food_y: 3
	}
    }

    pub fn reset(&mut self) {
        self.food_exist = true;
        self.game_over = false;
        self.food_x = 5;
        self.food_y = 3;
        self.snake.reset();
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
	self.snake.draw(con, g);
        
        if self.food_exist {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }
	draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
	draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
	draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
	draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
    }

    pub fn update(&mut self) {
        if !self.food_exist {
            self.add_food();
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
	let dir = match key {
	    Key::Up => Some(Direction::Up),
	    Key::Down => Some(Direction::Down),
	    Key::Left => Some(Direction::Left),
	    Key::Right => Some(Direction::Right),
	    _ => None
	};

	if dir.unwrap() == self.snake.head_direction().opposite() {
	    return;
	}

	self.update_snake(dir);
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_is_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exist && self.food_x == head_x && self.food_y == head_y {
            self.food_exist = false;
            self.snake.restore_last_removed();
        }
    }

    fn check_is_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y): (i32, i32) = self.snake.next_head_position(dir);
        if self.snake.is_overlap_except_tail(next_x, next_y) {
            return false;
        }
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height -1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.height - 1);
        while self.snake.is_overlap_except_tail(new_x, new_y) {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.height - 1);
        }
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exist = true;
    }
}

//https://github.com/SLMT/rust-snake/tree/master/src
