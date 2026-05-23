pub enum LinkType {
    Remote(String),
    Local(String),
    Ignored,
    Invalid(String),
}

pub fn classify_link(url: &str) -> LinkType {
    let trimmed = url.trim();

    if trimmed.is_empty() {
        return LinkType::Invalid("Empty URL".to_string());
    }

    if trimmed.starts_with("https://") || trimmed.starts_with("http://") {
        return LinkType::Remote(trimmed.to_string());
    }

    if trimmed.starts_with('#') {
        return LinkType::Ignored;
    }

    if trimmed.starts_with("//") {
        return LinkType::Remote(format!("https:{}", trimmed));
    }

    if trimmed.starts_with("mailto:")
        || trimmed.starts_with("tel:")
        || trimmed.starts_with("sms:")
        || trimmed.starts_with("javascript:")
    {
        return LinkType::Ignored;
    }

    LinkType::Local(trimmed.to_string())
}
