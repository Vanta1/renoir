// This file should serve as an example for game devs new to renoired, easy to follow

use nalgebra::Vector3;
use renoired::prelude::*;

fn main() {
    println!("WASD or the mouse to move, Press Ctrl+C or Q to close this game!");

    let mut player: (f32, f32, f32) = (0., 0., -10.);
    // TODO: add way to set up camera before running the game
    let speed: f32 = 3.;

    let mut game = RenoiredApp::new();

    game.run(move |rn| {
        // 'rn' is an abbreviation of Renoired here.

        rn.grab_cursor(true);

        if (rn.input.pressed(Key::C) && rn.input.pressed(Key::Ctrl)) || rn.input.pressed(Key::Q) {
            rn.close();
        }

        player.0 += ((rn.input.pressed(Key::D) as i32 - rn.input.pressed(Key::A) as i32) as f32)
            * speed
            * rn.time.delta_time();
        player.1 += ((rn.input.pressed(Key::Space) as i32 - rn.input.pressed(Key::Shift) as i32)
            as f32)
            * speed
            * rn.time.delta_time();
        player.2 -= ((rn.input.pressed(Key::W) as i32 - rn.input.pressed(Key::S) as i32) as f32)
            * speed
            * rn.time.delta_time();

        let mouse_move = rn.input.get_mouse_delta();

        rn.camera.rotate_y(mouse_move.0 / 100.0);
        rn.camera.rotate_x(-mouse_move.1 / 100.0);
        rn.camera.set_translate(player.0, player.1, player.2);

        println!("{}, {}", mouse_move.0, mouse_move.1);
    })
}
