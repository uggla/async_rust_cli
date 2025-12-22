use thiserror::Error;

#[derive(Error, Debug)]
pub enum SncfAPIError {
    #[error("API error: {0}")]
    ApiError(String),
}

// TODO: Fix the following enum
pub enum Call {
    Ok,
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
pub fn call_me(arg: Call) -> Result<(), SncfAPIError> {
    // TODO: Complete this function
    todo!()
}

// TODO: Change the signature to not move and borrow arg.
pub fn call_me_twice(arg: Call) -> Result<(), SncfAPIError> {
    // TODO: Copy the body from the call_me function
    todo!()
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
