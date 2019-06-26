extern crate common_voice_yotp;

use common_voice_yotp::errors::*;
use std::env;

use common_voice_yotp::app;

fn main() -> Result<()> {
    for o in app::run(env::args_os())? {
        println!("{}", o);
    }
    Ok(())
}
