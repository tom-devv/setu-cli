use core::fmt;
use std::path::PathBuf;

use colored::Colorize;

use crate::checker;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalLinkStatus {
    Valid,
    DoesNotExist,
    InvalidPrefix,
}

impl fmt::Display for LocalLinkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocalLinkStatus::Valid => write!(f, "The local link is valid"),
            LocalLinkStatus::DoesNotExist => write!(f, "The local link does not exist"),
            LocalLinkStatus::InvalidPrefix => {
                write!(f, "The local link's path is malformatted")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoteLinkStatus {
    Reachable,       // Could this be changed?
    Concern(u16),    // Status code - 404, 501 etc.
    Invalid(String), // Err message - not to be confused with 404 which is a Concern
}

impl fmt::Display for RemoteLinkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Concern(code) => {
                write!(f, "The URL returned an unsuccessful status code: {}", code)
            }
            Self::Invalid(err) => {
                write!(f, "The URL could not be reached: {}", err)
            }
            Self::Reachable => write!(f, "The URL returns 200 OK"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkStatus {
    Local(LocalLinkStatus),
    Remote(RemoteLinkStatus),
    Invalid(String),
}

impl LinkStatus {
    pub fn is_broken(&self) -> bool {
        match self {
            LinkStatus::Local(local_link_status) => match local_link_status {
                LocalLinkStatus::Valid => false,
                _ => true,
            },
            LinkStatus::Remote(remote_link_status) => match remote_link_status {
                RemoteLinkStatus::Reachable => false,
                RemoteLinkStatus::Concern(_) => true, // concerns are specified by user input so must be deemed broken if they are found
                _ => true,
            },
            _ => true,
        }
    }
}

impl fmt::Display for LinkStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkStatus::Local(status) => fmt::Display::fmt(status, f),
            LinkStatus::Remote(status) => fmt::Display::fmt(status, f),
            LinkStatus::Invalid(err) => fmt::Display::fmt(err, f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkCheckResult {
    pub source_file: PathBuf,
    pub raw_link: String,
    pub status: LinkStatus,
}

#[derive(Debug)]
pub struct MarkdownCheckResult {
    pub success: bool,
    pub checks: Vec<LinkCheckResult>,
}

impl fmt::Display for MarkdownCheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.success {
            for check in &self.checks {
                match &check.status {
                    checker::LinkStatus::Local(status) if *status != LocalLinkStatus::Valid => {
                        write!(
                            f,
                            "{} Faulty local link {} at {} | {}\n",
                            "FAIL".red().bold(),
                            check.raw_link.cyan(),
                            check.source_file.to_string_lossy().green(),
                            check.status
                        )?;
                    }
                    checker::LinkStatus::Remote(RemoteLinkStatus::Concern(_))
                    | checker::LinkStatus::Remote(RemoteLinkStatus::Invalid(_)) => {
                        write!(
                            f,
                            "{} Faulty remote URL {} at {} | {}\n",
                            "FAIL".red().bold(),
                            check.raw_link.cyan(),
                            check.source_file.to_string_lossy().green(),
                            check.status
                        )?;
                    }
                    checker::LinkStatus::Invalid(err) => {
                        write!(
                            f,
                            "{} Invalid URL {} at {} | {}\n",
                            "FAIL".red().bold(),
                            check.raw_link.cyan(),
                            check.source_file.to_string_lossy().green(),
                            err
                        )?;
                    }
                    _ => {}
                }
            }
        } else {
            write!(
                f,
                "{} Failed to locate/parse markdown file",
                "FAIL".red().bold()
            )?;
        }
        Ok(())
    }
}

pub mod invalid;
pub mod local;
pub mod remote;
