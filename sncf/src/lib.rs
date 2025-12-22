use thiserror::Error;

#[derive(Error, Debug)]
pub enum SncfAPIError {
    #[error("API error: {0}")]
    ApiError(String),
}

pub enum Call {
    Ok,
    Ko,
}

pub fn call_me(arg1: Call) -> Result<(), SncfAPIError> {
    match arg1 {
        Call::Ok => Ok(()),
        Call::Ko => Err(SncfAPIError::ApiError("This call fails".to_string())),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
}
