use anyhow::bail;
// TODO: Fix the following import
use sncf::{Call, call_me, call_me_twice};

pub const APPNAME: &str = env!("CARGO_PKG_NAME");

pub fn run() -> anyhow::Result<()> {
    let arg1 = Call::Ok;

    call_me(arg1)?;

    let arg2 = Call::Ko;
    // TODO: Fix to call the call_me_function (do not move arg2)
    let _ = call_me_twice(arg2);
    let _ = call_me_twice(arg2);
    Ok(())
}

// TODO: Fix the visibility of the function
fn api_check(api: String) -> anyhow::Result<()> {
    match api.as_str() {
        "change_me" => Ok(()),
        _ => bail!("Wrong api key"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fake() {
        assert_eq!(1, 1);
    }

    #[test]
    #[should_panic(expected = "Houston, we have a problem !")]
    fn fake_panic() {
        panic!("Houston, we have a problem !");
    }

    #[test]
    fn run_calls_sncf_ok() {
        let result = run();

        assert!(result.is_ok());
    }

    #[test]
    fn api_check_accepts_expected_key() {
        let result = api_check("change_me".to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn api_check_rejects_other_keys() {
        let result = api_check("nope".to_string());

        let err = result.expect_err("expected api_check to fail for invalid key");
        assert_eq!(err.to_string(), "Wrong api key");
    }
}
