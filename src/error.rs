use reqwest;
use std::convert::From;

#[derive(Debug)]
pub enum SimonError {
    HttpError(reqwest::Error),
}

impl From<reqwest::Error> for SimonError {
    fn from(e: reqwest::Error) -> SimonError {
        SimonError::HttpError(e)
    }
}
