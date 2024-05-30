// This file should serve as an example for game devs new to renoired, easy to follow

use renoired::prelude::*;

fn main() {
    println!("WASD to move, Press Ctrl+C or Q to close this game!");

    let mut player: (f64, f64) = (4., 5.);
    let speed: f64 = 3.;

    let mut game = RenoiredApp::new();

    game.run(move |game| {
        alias!(game, input, time, flow);

        if (input.pressed(Key::C) && input.pressed(Key::Ctrl)) || input.pressed(Key::Q) {
            flow.close();
        }

        player.0 += ((input.pressed(Key::D) as i32 - input.pressed(Key::A) as i32) as f64) * speed * time.delta_time();
        player.1 += ((input.pressed(Key::W) as i32 - input.pressed(Key::S) as i32) as f64) * speed * time.delta_time();

        println!("X: {}, Y: {}", player.0, player.1);
    })
}
