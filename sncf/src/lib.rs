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

/// Calls the SNCF API simulation.
///
/// # Examples
///
/// ```rust
/// use sncf::{call_me, Call};
///
/// assert!(call_me(Call::Ok).is_ok());
/// assert!(call_me(Call::Ko).is_err());
/// ```
pub fn call_me(arg1: Call) -> Result<(), SncfAPIError> {
    match arg1 {
        Call::Ok => Ok(()),
        Call::Ko => Err(SncfAPIError::ApiError("This call fails".to_string())),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn call_me_ok_returns_ok() {
        let result = call_me(Call::Ok);

        assert!(result.is_ok());
    }

    #[test]
    fn call_me_ko_returns_api_error() {
        let result = call_me(Call::Ko);

        let err = result.expect_err("expected error for Call::Ko");
        assert_eq!(err.to_string(), "API error: This call fails");
    }
}
