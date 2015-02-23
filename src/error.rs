use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy)]
pub enum BeanstalkdError { ConnectionError, RequestError }

impl Error for BeanstalkdError {
    fn description(&self) -> &str {
        match *self {
            BeanstalkdError::ConnectionError => "Connection error occurred",
            BeanstalkdError::RequestError => "Request error occurred",
        }
    }
}

impl Display for BeanstalkdError {
    fn fmt(&self, formatter: &mut Formatter) -> ::std::fmt::Result {
        self.description().fmt(formatter)
    }
}

pub type BeanstalkdResult<T> = Result<T, BeanstalkdError>;
