use piston_window::{Context, G2d};
use piston_window::types::Color;

use std::collections::LinkedList;

use crate::drawing::draw_block;

const SNAKE_COLOR: Color = [0.18, 0.80, 0.44, 1.0];

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    pub fn opposite(&self) -> Direction {
	match *self {
	    Direction::Up => Direction::Down,
	    Direction::Down => Direction::Up,
	    Direction::Left => Direction::Right,
	    Direction::Right => Direction::Left
	}
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    x: i32,
    y: i32
}

pub struct Snake {
    moving_direction: Direction,
    body: LinkedList<Block>,
    last_removed_block: Option<Block>
}

impl Snake {
    pub fn new(init_x: i32, init_y: i32) -> Snake {
	let mut body: LinkedList<Block> = LinkedList::new();
	body.push_back(Block {
	    x: init_x + 2,
	    y: init_y
	});
	body.push_back(Block {
	    x: init_x + 1,
	    y: init_y
	});
	body.push_back(Block {
	    x: init_x,
	    y: init_y
	});

	Snake {
	    moving_direction: Direction::Right,
	    body: body,
	    last_removed_block: None
	}
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
	for block in &self.body {
	    draw_block(SNAKE_COLOR, block.x, block.y, con, g);
	}
    }

    pub fn head_position(&self) -> (i32, i32) {
	let head_block = self.body.front().unwrap();
	(head_block.x, head_block.y)
    }

    pub fn head_direction(&self) -> Direction {
	self.moving_direction
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
	match dir {
	    Some(d) => self.moving_direction = d,
	    None => {}
	}

	let (last_x, last_y) = self.head_position();

	let new_block = match self.moving_direction {
	    Direction::Up => Block {
		x: last_x,
		y: last_y - 1
	    },
	    Direction::Right => Block {
		x: last_x + 1,
		y: last_y
	    },
	    Direction::Down => Block {
		x: last_x,
		y: last_y + 1
	    },
	    Direction::Left => Block {
		x: last_x - 1,
		y: last_y
	    }
	};
	self.body.push_front(new_block);
	let removed_blk = self.body.pop_back().unwrap();
        self.last_removed_block = Some(removed_blk);
    }

    pub fn is_overlap_except_tail(&self, x: i32, y: i32) -> bool {
        let mut checked = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true
            }
            checked += 1;
            if checked == self.body.len() -1 {
                break;
            }
        }
        return false
    }
    
    pub fn restore_last_removed(&mut self) {
        let blk = self.last_removed_block.clone().unwrap();
        self.body.push_back(blk);
    }
}
