// This file should serve as an example for game devs new to Renoir, easy to follow

use renoir::prelude::*;

fn main() {
    println!("WASD or the mouse to move, Press Ctrl+C or Q to close this game!");

    let mut player = Vec3::new(0., 3., 8.);
    // TODO: add way to set up camera before running the game
    let speed: f32 = 3.;

    let mut game = RenoirApp::new();

    game.window_options(WindowOptions {
        fullscreen: true,
        grab_cursor: true,
        show_cursor: false,
    });

    // 'rn' is an abbreviation of Renoir here.
    game.run(move |rn| {
        if (rn.input.pressed(Key::C) && rn.input.pressed(Key::Ctrl)) || rn.input.pressed(Key::Q) {
            rn.close();
        }

        player.x += ((rn.input.pressed(Key::D) as i32 - rn.input.pressed(Key::A) as i32) as f32)
            * speed
            * rn.time.delta_time();
        player.y += ((rn.input.pressed(Key::Space) as i32 - rn.input.pressed(Key::Shift) as i32)
            as f32)
            * speed
            * rn.time.delta_time();
        player.z -= ((rn.input.pressed(Key::W) as i32 - rn.input.pressed(Key::S) as i32) as f32)
            * speed
            * rn.time.delta_time();

        let mouse_move = rn.input.get_mouse_delta();

        rn.camera.rotate_y(mouse_move.0 / 100.0);
        rn.camera.rotate_x(-mouse_move.1 / 100.0);
        rn.camera.set_translate(player.x, player.y, player.z);
    })
}
