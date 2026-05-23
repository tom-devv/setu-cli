use std::{fs, path::PathBuf};

use pulldown_cmark::{Event, Parser, Tag, TextMergeStream};
use reqwest::Client;

use crate::{checker::MarkdownCheckResult, parser::classifier::classify_link};

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
    let mut invalids: Vec<String> = Vec::new();

    for event in iterator {
        if let Event::Start(Tag::Link { dest_url, .. }) = event {
            let link = dest_url.to_string();
            match classify_link(&link) {
                super::classifier::LinkType::Remote(url) => remotes.push(url),
                super::classifier::LinkType::Local(url) => locals.push(url),
                super::classifier::LinkType::Ignored => {}
                super::classifier::LinkType::Invalid(url) => invalids.push(url),
            }
        }
    }
    let client = Client::new();
    let remote_checks = crate::checker::remote::evaluate(path, remotes, &client, concerns).await;
    let local_checks = crate::checker::local::evaluate(path, locals);
    let invalid_checks = crate::checker::invalid::evaluate(path, invalids);

    let checks = local_checks
        .into_iter()
        .chain(remote_checks)
        .chain(invalid_checks)
        .collect();

    MarkdownCheckResult {
        success: true,
        checks,
    }
}
