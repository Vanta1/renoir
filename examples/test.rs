// This file should serve as an example for game devs new to renoired, easy to follow

use renoired::prelude::*;

fn main() {
    println!("WASD to move, Press Ctrl+C or Q to close this game!");

    let mut player = (4., 5.);
    let speed = 3.;

    let mut game = RenoiredApp::new();

    game.run(move |game| {
        alias!(game, input, time, flow);

        if (input.pressed(Key::C) && input.pressed(Key::Ctrl)) || input.pressed(Key::Q) {
            flow.close();
        }

        if input.pressed(Key::W) {
            player.1 += speed * time.delta_time()
        }
        if input.pressed(Key::S) {
            player.1 -= speed * time.delta_time()
        }
        if input.pressed(Key::A) {
            player.0 -= speed * time.delta_time()
        }
        if input.pressed(Key::D) {
            player.0 += speed * time.delta_time()
        }

        //println!("X: {}, Y: {}", player.0, player.1);
    })
}
