#![no_std]

use crankit_game_loop::{game_loop, Game, Playdate};

pub struct MyGame;

impl Game for MyGame {
    fn new(_: &Playdate) -> Self {
        Self
    }

    fn update(&mut self, _: &Playdate) {}
}

game_loop!(MyGame);
