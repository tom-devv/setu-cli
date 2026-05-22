use setu_cli::{
    LinkCheckResult, LinkStatus, LocalLinkStatus, MarkdownCheckResult, RemoteLinkStatus,
    get_markdowns, parse_markdown,
};

#[test]
fn test_get_markdowns() {
    let markdowns = get_markdowns("./tests");

    assert!(markdowns.iter().count() == 1);
}

#[tokio::test]
async fn test_parse_markdown_no_concerns() {
    let markdowns = get_markdowns("./tests");

    assert_eq!(markdowns.len(), 1);

    let result = parse_markdown(markdowns.first().unwrap(), &Some(vec![])).await;

    assert!(result.success, "The file parsing operation itself failed");
    assert_eq!(
        result.checks.len(),
        5,
        "Should have found exactly 5 links in test.md"
    );

    let valid_local_check = get_result(
        &result,
        "./real.txt".to_owned(),
        "Could not find the link to ./real.txt in results",
    );

    assert!(
        matches!(
            valid_local_check.status,
            LinkStatus::Local(LocalLinkStatus::Valid)
        ),
        "Expected ./real.txt to be classified as a Valid Local link"
    );

    let broken_local_check = get_result(
        &result,
        "./does_not_exist.png".to_owned(),
        "Could not find the link to ./does_not_exist.png in results",
    );
    assert!(
        matches!(
            broken_local_check.status,
            LinkStatus::Local(LocalLinkStatus::DoesNotExist)
        ),
        "Expected ./does_not_exist.png to be flagged as an Invalid Local link"
    );

    let remote_check_ok = get_result(
        &result,
        "https://www.google.com".to_owned(),
        "Could not find the link to google in results",
    );

    if let LinkStatus::Remote(RemoteLinkStatus::Invalid(err)) = &remote_check_ok.status {
        panic!("Google link failed unexpectedly with status: {}", err);
    }

    let remote_check_404 = get_result(
        &result,
        "https://simulatehttpcode.vercel.app/statuscode?q=404".to_owned(),
        "Could not find the link to the 404 page in results",
    );

    if let LinkStatus::Remote(RemoteLinkStatus::Invalid(err)) = &remote_check_404.status {
        panic!("404 link failed unexpectedly with status: {}", err);
    }
}

#[tokio::test]
async fn test_parse_markdown_with_404_concerns() {
    let markdowns = get_markdowns("./tests");

    assert_eq!(markdowns.len(), 1);

    let concerns = vec![404];

    let result = parse_markdown(markdowns.first().unwrap(), &Some(concerns.clone())).await;

    let remote_check_ok = get_result(
        &result,
        "https://www.google.com".to_owned(),
        "Could not find the link to google in results",
    );

    if let LinkStatus::Remote(RemoteLinkStatus::Invalid(err)) = &remote_check_ok.status {
        panic!("Google link failed unexpectedly with status: {}", err);
    }

    // This should fail
    let remote_check_404 = get_result(
        &result,
        "https://simulatehttpcode.vercel.app/statuscode?q=404".to_owned(),
        "Could not find the link to the 404 page in results",
    );

    if let LinkStatus::Remote(RemoteLinkStatus::Reachable) = &remote_check_404.status {
        panic!(
            "404 link failed unexpectedly with concerns: {:?}",
            &concerns
        );
    }
}

#[tokio::test]
async fn test_parse_markdown_with_many_concerns() {
    let markdowns = get_markdowns("./tests");

    assert_eq!(markdowns.len(), 1);

    let concerns = vec![404, 500];

    let result = parse_markdown(markdowns.first().unwrap(), &Some(concerns.clone())).await;

    let remote_check_ok = get_result(
        &result,
        "https://www.google.com".to_owned(),
        "Could not find the link to google in results",
    );

    if let LinkStatus::Remote(RemoteLinkStatus::Invalid(err)) = &remote_check_ok.status {
        panic!("Google link failed unexpectedly with status: {}", err);
    }

    let remote_check_404 = get_result(
        &result,
        "https://simulatehttpcode.vercel.app/statuscode?q=404".to_owned(),
        "Could not find the link to the 404 page in results",
    );

    if let LinkStatus::Remote(RemoteLinkStatus::Reachable) = &remote_check_404.status {
        panic!(
            "404 link failed unexpectedly with concerns: {:?}",
            &concerns
        );
    }

    let remote_check_500 = get_result(
        &result,
        "https://simulatehttpcode.vercel.app/statuscode?q=500".to_owned(),
        "Could not find the link to the 500 page in results",
    );

    if let LinkStatus::Remote(RemoteLinkStatus::Reachable) = &remote_check_500.status {
        panic!(
            "500 link failed unexpectedly with concerns: {:?}",
            &concerns
        );
    }
}

fn get_result(result: &MarkdownCheckResult, raw_link: String, expect: &str) -> LinkCheckResult {
    result
        .checks
        .iter()
        .find(|c| c.raw_link == raw_link)
        .expect(expect)
        .clone()
}
