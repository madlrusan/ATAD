use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrawlerError {
    #[error("Invalid URL")]
    InvalidUrl,
    #[error("Failed to fetch URL: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse HTML")]
    ParseError,
}
