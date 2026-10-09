use clap::{Parser, Subcommand};

use crate::extractor::extract;
use crate::loaders::{File, Wikipedia};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output all the sentences without verification
    #[arg(short, long)]
    no_check: bool,

    /// Language as identified by ISO code - for example en, de, es
    #[arg(short, long)]
    language: String,

    /// Path to folder with files to process
    #[arg(short, long)]
    dir: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Extract sentences from Wikipedia dump extracts using WikiExtractor
    Extract {
        /// path to the file containing titles to filter for
        #[arg(short, long)]
        title_filter_list: Option<String>,
    },

    /// Extract sentences from Wikisource dump extracts using WikiExtractor
    ExtractWikisource,

    /// Extract sentences from files which have one sentence per line
    ExtractFile,
}

pub fn start() -> Result<(), String> {
    let args = Args::parse();

    let no_check = args.no_check;
    let language = args.language;
    let directory = args.dir;

    match &args.command {
        Commands::Extract { title_filter_list } => {
            let wikipedia_loader = Wikipedia::new(language, directory);
            let filter_list_value = title_filter_list.clone().unwrap_or(String::from(""));
            extract(wikipedia_loader, no_check, filter_list_value)
        },
        Commands::ExtractWikisource => {
            let wikipedia_loader = Wikipedia::new(language, directory);
            extract(wikipedia_loader, no_check, String::from(""))
        },
        Commands::ExtractFile => {
            let file_loader = File::new(language, directory);
            extract(file_loader, no_check, String::from(""))
        }
    }
}
