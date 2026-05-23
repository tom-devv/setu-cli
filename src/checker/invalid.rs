use std::path::PathBuf;

use crate::checker::{LinkCheckResult, LinkStatus};

fn evaluate_invalid_path(mdx_path: &PathBuf, invalid_path: &String) -> LinkCheckResult {
    LinkCheckResult {
        source_file: mdx_path.clone(),
        raw_link: invalid_path.clone(),
        status: LinkStatus::Invalid("Invalid Path".to_owned()),
    }
}

pub fn evaluate(mdx_path: &PathBuf, invalid_paths: Vec<String>) -> Vec<LinkCheckResult> {
    invalid_paths
        .iter()
        .map(|invalid| evaluate_invalid_path(mdx_path, invalid))
        .collect()
}
