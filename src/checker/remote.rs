use std::{path::PathBuf, time::Duration};

use futures::StreamExt;
use reqwest::{Client, StatusCode};

use crate::checker::{LinkCheckResult, LinkStatus, RemoteLinkStatus};

async fn evaluate_remote_url(
    mdx_path: PathBuf,
    remote_url: String,
    client: Client,
    concerns: &Option<Vec<u16>>,
) -> LinkCheckResult {
    let response = client
        .head(&remote_url)
        .timeout(Duration::from_secs(5))
        .send()
        .await;

    match response {
        Ok(res) => {
            // Concerning URL's are matched against program arguments
            let status = if res.status() == StatusCode::OK {
                RemoteLinkStatus::Reachable
            } else {
                let return_code = res.status().as_u16();
                match concerns {
                    Some(codes) if codes.contains(&return_code) => {
                        RemoteLinkStatus::Concern(return_code)
                    }
                    _ => RemoteLinkStatus::Reachable,
                }
            };
            LinkCheckResult {
                source_file: mdx_path,
                raw_link: remote_url,
                status: LinkStatus::Remote(status),
            }
        }
        Err(err) => LinkCheckResult {
            source_file: mdx_path.clone(),
            raw_link: remote_url.to_owned(),
            status: LinkStatus::Remote(RemoteLinkStatus::Invalid(err.to_string())),
        },
    }
}

pub async fn evaluate(
    mdx_path: &PathBuf,
    remote_urls: Vec<String>,
    client: &Client,
    concerns: &Option<Vec<u16>>,
) -> Vec<LinkCheckResult> {
    const MAX_CONCURRENT_REQUESTS: usize = 50;

    let worker_stream = futures::stream::iter(remote_urls)
        .map(|url| {
            let client = client.clone();
            let mdx_path = mdx_path.clone();
            async move { evaluate_remote_url(mdx_path, url, client, concerns).await }
        })
        .buffer_unordered(MAX_CONCURRENT_REQUESTS);

    let results: Vec<LinkCheckResult> = worker_stream.collect().await;
    results
}
