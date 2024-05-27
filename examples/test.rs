// This file should serve as an example for game devs new to renoired, easy to follow

use renoired::prelude::*;

fn main() {
    println!("Press Ctrl+C to close this game!");

    let mut player = (4., 5.);
	let speed = 3.;

    RenoiredApp::new().run(move |game| {
        if game.input.pressed(Key::C) && game.input.pressed(Key::Ctrl) {
            game.close();
        }

		if game.input.pressed(Key::W) { player.1 += speed * game.time.delta_time() }
        if game.input.pressed(Key::S) { player.1 -= speed * game.time.delta_time() }
        if game.input.pressed(Key::A) { player.0 -= speed * game.time.delta_time() }
        if game.input.pressed(Key::D) { player.0 += speed * game.time.delta_time() }

        dbg!(player);
    })
}
