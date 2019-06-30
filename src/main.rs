extern crate piston_window;

use piston_window::*;


mod drawing;
mod game;
mod snake;


use game::Game;
use drawing::to_gui_coord_u32;


fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!",
	[to_gui_coord_u32(width), to_gui_coord_u32(height)])
	.exit_on_esc(true).build().unwrap();
    
    let mut game = Game::new(20, 20);
    while let Some(event) = window.next() {
        if game.game_over == true {
	    game.reset();
	}
	
	if let Some(Button::Keyboard(key)) = event.press_args() {
	    game.key_pressed(key);
	}
	
	window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            game.draw(&context, graphics);
        });

        event.update(|arg| {
            game.update();
        });
    }
}

