use std::sync::{RwLock, RwLockReadGuard};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Order & Chaos", about = "An emulator for the classic two-player board game \"Order and Chaos\"")]
pub struct Options {
    #[structopt(long = "demo", help = "Shows a demo of the game with random moves.")]
    pub ai_vs_ai_demo: bool,

    #[structopt(short = "t", long="terminal", help = "Disables The Graphical User Interface and relies on a terminal console.")]
    pub disable_gui: bool,

    #[structopt(long, help = "Disables Emoji output. Only applicable if --terminal is also set")]
    pub disable_emoji: bool,

    #[structopt(long, help = "Disables Emoji and Colored text output. Only applicable if --terminal is also set")]
    pub disable_color_and_emoji: bool,
}

impl Options {
    pub const fn default_options() -> Self {
        Self {
            ai_vs_ai_demo: false,
            disable_emoji: false,
            disable_color_and_emoji: false,
            disable_gui: false,
        }
    }
}

static GLOBAL_OPTIONS: RwLock<Options> = RwLock::new(Options::default_options());

pub fn read_options() {
    let mut options_w = GLOBAL_OPTIONS.write().unwrap();
    *options_w = Options::from_args();
}

pub fn get() -> RwLockReadGuard<'static, Options> {
    return GLOBAL_OPTIONS.read().unwrap();
}