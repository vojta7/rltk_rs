// Platform to integrate into Amethyst
pub mod font;
mod init;
pub mod shader;
pub use init::*;
mod mainloop;
pub use mainloop::*;
mod dummy;
pub use dummy::*;

pub struct InitHints {
    pub vsync: bool,
    pub fullscreen: bool,
}

impl InitHints {
    pub fn new() -> Self {
        Self {
            vsync: true,
            fullscreen: false,
        }
    }
}

pub fn log(s: &str) {
    println!("{}", s);
}