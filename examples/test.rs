// This file should serve as an example for game devs new to renoired, easy to follow

use renoired::prelude::*;

fn main() {
    println!("WASD or the mouse to move, Press Ctrl+C or Q to close this game!");

    let mut player: (f64, f64) = (4., 5.);
    let speed: f64 = 3.;

    let mut game = RenoiredApp::new();

    game.run(move |rn| {
        // 'rn' is an abbreviation of Renoired here.
        
        rn.grab_cursor(true);

        if (rn.input.pressed(Key::C) && rn.input.pressed(Key::Ctrl)) || rn.input.pressed(Key::Q) {
            rn.close();
        }

        player.0 += ((rn.input.pressed(Key::D) as i32 - rn.input.pressed(Key::A) as i32) as f64)
            * speed
            * rn.time.delta_time();
        player.1 += ((rn.input.pressed(Key::W) as i32 - rn.input.pressed(Key::S) as i32) as f64)
            * speed
            * rn.time.delta_time();

        let mouse_move = rn.input.get_mouse_delta();
        player.0 += mouse_move.0 * speed * rn.time.delta_time();
        player.1 -= mouse_move.1 * speed * rn.time.delta_time();

        println!("X: {}, Y: {}", player.0, player.1);
    })
}
