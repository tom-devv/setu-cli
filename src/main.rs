use std::process;

use colored::Colorize;
use futures::future::join_all;

use clap::Parser;
use setu_cli::{MarkdownCheckResult, get_markdowns, parse_markdown};

#[derive(Parser, Debug)]
#[command(name = "setu")]
#[command(author, version, about = "Setu checks markdown for broken links", long_about = None)]
struct SetuArgs {
    #[arg(default_value = ".")]
    target_path: String,

    #[arg(short, long, value_delimiter = ',', default_value = "404")]
    concerns: Option<Vec<u16>>,

    #[arg(short, long)]
    strict: bool,
}

#[tokio::main]
async fn main() {
    let setu_args = SetuArgs::parse();

    let paths = get_markdowns(&setu_args.target_path);

    let tasks = paths
        .iter()
        .map(|path| parse_markdown(path, &setu_args.concerns));

    let results: Vec<MarkdownCheckResult> = join_all(tasks).await;

    for result in &results {
        println!("{}\n", result);
    }

    let malformed_links = results
        .into_iter()
        .filter(|md| md.success)
        .flat_map(|res| res.checks)
        .filter(|check| check.status.is_broken())
        .count()
        > 0;

    if malformed_links {
        println!("{}", "Scan complete. Broken links detected".red().bold());

        if setu_args.strict {
            process::exit(1)
        }
    } else {
        println!("{}", "Scan complete. All links healthy!".green().bold());
    }
}
