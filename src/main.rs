extern crate common_voice_yotp;

use std::env;

use common_voice_yotp::app;

fn main() -> Result<(), String> {
    app::run(env::args_os())
}
