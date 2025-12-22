use anyhow::bail;
use sncf::{Call, call_me};

pub fn run() -> anyhow::Result<()> {
    let arg1 = Call::Ok;

    call_me(arg1)?;
    Ok(())
}

pub fn api_check(api: String) -> anyhow::Result<()> {
    match api.as_str() {
        "change_me" => Ok(()),
        _ => bail!("Wrong api key"),
    }
}
