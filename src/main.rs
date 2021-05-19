extern crate common_voice_sentence_collector;

use std::env;

use common_voice_sentence_collector::app;

fn main() -> Result<(), String> {
    app::run(env::args_os())
}
