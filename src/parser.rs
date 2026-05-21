use std::{fs, path::PathBuf};

use pulldown_cmark::{Event, Parser, Tag, TextMergeStream};
use reqwest::Client;

use crate::checker::MarkdownCheckResult;

pub async fn parse_markdown(path: &PathBuf, concerns: &Option<Vec<u16>>) -> MarkdownCheckResult {
    let markdown_input = match fs::read_to_string(&path) {
        Ok(contents) => contents.to_string(),
        Err(_) => {
            return MarkdownCheckResult {
                success: false,
                checks: vec![],
            };
        }
    };

    let iterator = TextMergeStream::new(Parser::new(&markdown_input));
    let mut remotes: Vec<String> = Vec::new();
    let mut locals: Vec<String> = Vec::new();

    for event in iterator {
        if let Event::Start(Tag::Link { dest_url, .. }) = event {
            let destination = dest_url.to_string();
            if is_remote_url(&destination) {
                remotes.push(destination);
            } else {
                locals.push(destination);
            }
        }
    }
    let client = Client::new();
    let remote_checks = crate::checker::remote::evaluate(path, remotes, &client, concerns).await;
    let local_checks = crate::checker::local::evaluate(path, locals);

    let checks = local_checks.into_iter().chain(remote_checks).collect();

    MarkdownCheckResult {
        success: true,
        checks,
    }
}

fn is_remote_url(url: &str) -> bool {
    url.starts_with("https://") || url.starts_with("http://")
}
