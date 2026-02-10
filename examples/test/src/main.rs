// This file should serve as an example for game devs new to Renoir, easy to follow

use renoir::prelude::*;

enum ObjType {
    Player,
    Entity,
}

fn main() {
    println!("WASD or the mouse to move, Press Ctrl+C or Q to close this game!");

    let mut player = Vec3::new(0., 3., 8.);
    // TODO: add way to set up camera before running the game
    let speed: f32 = 3.;

    let mut game = RenoirApp::new();

    // 'ren' is a mutable reference to a RenoirAppState, which is the main way of interacting with the engine.
    // it can be queried (e.g. ren.input.pressed(Key::C)) or edited (ren.camera.set_translate())
    // RenoirApp::setup is run once at the beginning
    // TODO: I could just refactor the "setup" stage to be an intermediate function, e.g. the user (dev) calls game.run(), within which
    // they call ren.run(), which starts the actual game. this pattern can be seen in zed's GPUI crate
    game.setup(|ren| {
        println!("Setting up...");
        ren.window_options = WindowOptions {
            fullscreen: true,
            grab_cursor: true,
            show_cursor: false,
        };

        // available because of the "ecs" feature
        ren.world.spawn((Vec3::new(0., 0., 0.), ObjType::Player));
        ren.world.spawn((Vec3::new(0., 0., 0.), ObjType::Entity));
    });

    game.quit(|_| println!("Exiting..."));

    // RenoirApp::run is run every frame
    game.run(move |ren| {
        if (ren.input.pressed(Key::C) && ren.input.pressed(Key::Ctrl)) || ren.input.pressed(Key::Q)
        {
            ren.close();
        }

        player.x += ((ren.input.pressed(Key::D) as i32 - ren.input.pressed(Key::A) as i32) as f32)
            * speed
            * ren.time.delta_time();
        player.y += ((ren.input.pressed(Key::Space) as i32 - ren.input.pressed(Key::Shift) as i32)
            as f32)
            * speed
            * ren.time.delta_time();
        player.z -= ((ren.input.pressed(Key::W) as i32 - ren.input.pressed(Key::S) as i32) as f32)
            * speed
            * ren.time.delta_time();

        let mouse_move = ren.input.get_mouse_delta();

        ren.camera.rotate_y(mouse_move.0 / 100.0);
        ren.camera.rotate_x(-mouse_move.1 / 100.0);
        ren.camera.set_translate(player.x, player.y, player.z);
    })
}
