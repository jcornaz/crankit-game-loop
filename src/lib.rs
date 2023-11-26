#![no_std]

//! A safe and ergonomic entry-point for playdate games.
//!
//! To use this crate, implement the [`Game`] trait, and call the [`game_loop!`] macro.
//!
//! ## Examples
//!
//! ```
//! struct MyGame;
//! impl crankit_game_loop::Game for MyGame {
//!    fn new(_playdate: &crankit_game_loop::ffi::PlaydateAPI) -> Self {
//!        // Initialize your game here
//!       Self
//!    }
//!    fn update(&mut self, _playdate: &crankit_game_loop::ffi::PlaydateAPI) {
//!       // Update and render your game here
//!    }
//! }
//! ```

pub mod ffi {
    //! Re-exports from the crate [playdate-sys](https://docs.rs/playdate-sys)
    pub use playdate_sys::{
        ffi::{
            playdate_graphics as Graphics, playdate_sys as System, PDSystemEvent as SystemEvent,
            PlaydateAPI,
        },
        ll_symbols, EventLoopCtrl,
    };
}

#[non_exhaustive]
pub struct Playdate {
    pub playdate: &'static ffi::PlaydateAPI,
    pub system: &'static ffi::System,
    pub graphics: &'static ffi::Graphics,
}

pub trait Game {
    /// Invoked once at startup
    ///
    /// This is a good place to load images/sounds, initialize the game state
    /// As well as configuring the game (i.e. FPS)
    fn new(playdate: &ffi::PlaydateAPI) -> Self;

    /// Invoked every frame
    ///
    /// This is where you update you game state and render the new frame.
    fn update(&mut self, playdate: &ffi::PlaydateAPI);
}

#[macro_export]
macro_rules! game_loop {
    ($game_type:tt) => {
        mod __playdate_game {
            static mut PLAYDATE: Option<&'static $crate::ffi::PlaydateAPI> = None;
            static mut GAME: Option<super::$game_type> = None;

            #[no_mangle]
            fn event_handler(
                api: core::ptr::NonNull<$crate::ffi::PlaydateAPI>,
                event: $crate::ffi::SystemEvent,
                _: u32,
            ) -> $crate::ffi::EventLoopCtrl {
                if event == $crate::ffi::SystemEvent::kEventInit {
                    unsafe {
                        let api = api.as_ref();
                        PLAYDATE = Some(api);
                        GAME = Some($crate::Game::new(api));
                        (*api.system).setUpdateCallback.unwrap()(
                            Some(update),
                            core::ptr::null_mut(),
                        );
                    }
                }
                $crate::ffi::EventLoopCtrl::Continue
            }

            extern "C" fn update(_user_data: *mut core::ffi::c_void) -> i32 {
                unsafe {
                    let playdate = PLAYDATE.as_ref().unwrap();
                    $crate::Game::update(GAME.as_mut().unwrap(), playdate);
                };
                1
            }

            $crate::ffi::ll_symbols!();
        }
    };
}
