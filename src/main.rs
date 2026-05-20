use std::process;

use colored::Colorize;
use futures::future::join_all;

use crate::{checker::MarkdownCheckResult, finder::get_markdowns, parser::parse_markdown};
use clap::Parser;

mod checker;
mod finder;
mod parser;

#[derive(Parser, Debug)]
#[command(name = "setu")]
#[command(author, version, about = "Setu checks markdown for broken links", long_about = None)]
struct Args {
    #[arg(default_value = ".")]
    target_path: String,

    #[arg(short, long)]
    strict: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let paths = get_markdowns(&args.target_path);

    let tasks = paths.iter().map(|path| parse_markdown(path));

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

        if args.strict {
            process::exit(1)
        }
    } else {
        println!("{}", "Scan complete. All links healthy!".green().bold());
    }
}
