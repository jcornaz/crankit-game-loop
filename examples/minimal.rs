#![no_std]

use crankit_game_loop::{game_loop, Game};

pub struct MyGame;

impl Game for MyGame {
    fn new(_: &crankit_game_loop::ffi::PlaydateAPI) -> Self {
        Self
    }

    fn update(&mut self, _: &crankit_game_loop::ffi::PlaydateAPI) {}
}

game_loop!(MyGame);
