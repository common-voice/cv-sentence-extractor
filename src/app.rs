use clap::{Parser, Subcommand};

use crate::extractor::extract;
use crate::loaders::{File, Wikipedia};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
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
        title_filter_list: String,
    },

    /// Extract sentences from Wikisource dump extracts using WikiExtractor
    ExtractWikisource,

    /// Extract sentences from files which have one sentence per line
    ExtractFile,
}

pub fn start() -> Result<(), String> {
    let cli = CLI::parse();

    let no_check = cli.no_check;
    let language = cli.language;
    let directory = cli.dir;

    match &cli.command {
        Commands::Extract { title_filter_list } => {
            let wikipedia_loader = Wikipedia::new(language, directory);
            return extract(wikipedia_loader, no_check, &title_filter_list);
        },
        Commands::ExtractWikisource => {
            let wikipedia_loader = Wikipedia::new(language, directory);
            return extract(wikipedia_loader, no_check, "");
        },
        Commands::ExtractFile => {
            let file_loader = File::new(language, directory);
            return extract(file_loader, no_check, "");
        }
    }
}
