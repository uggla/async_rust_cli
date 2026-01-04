use std::time::Duration;

use anyhow::bail;
// use sncf::{Call, call_me, call_me_twice};
use tokio::sync::mpsc::{self, error};

pub const APPNAME: &str = env!("CARGO_PKG_NAME");

#[allow(unused)]
pub async fn run(_api_key: String) -> anyhow::Result<()> {
    let mut count = 0;
    let mut msg = String::new();
    let (data_sender, mut data_receiver) = mpsc::channel::<String>(5);

    // Spawn a task here that will send data from the API.
    let refresh_task = tokio::spawn(async move {
        tracing::info!("refresh task started");

        loop {
            tracing::info!("sending data");
            if count == 5 {
                msg = "stop".to_string()
            } else {
                msg = format!("Hello {count}");
            }
            if let Err(e) = data_sender.send(msg).await {
                tracing::error!("Error sending message: {e}");
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
            count += 1;
        }

        tracing::error!("refresh task terminated");
    });

    // Following loop simulate the TUI main loop.
    loop {
        // Check that the external task is running, if not return an error.
        if refresh_task.is_finished() {
            return refresh_task_result_to_err(refresh_task.await);
        }

        // Manage message from refresh task
        match data_receiver.try_recv() {
            Err(error::TryRecvError::Empty) => {}
            Err(error::TryRecvError::Disconnected) => {}
            Ok(data) => {
                tracing::info!("data received");
                tracing::debug!("Data: {data}");
                // For testing purpose, we send a stop message after 5 iterations to go out of the
                // loop.
                if data == "stop" {
                    break;
                }
            }
        };
        tokio::time::sleep(Duration::from_millis(100)).await
    }
    Ok(())
}

pub fn api_check(api: String) -> anyhow::Result<()> {
    match api.as_str() {
        "change_me" => Ok(()),
        _ => bail!("Wrong api key"),
    }
}

fn refresh_task_result_to_err(res: Result<(), tokio::task::JoinError>) -> anyhow::Result<()> {
    match res {
        Ok(_) => {
            tracing::warn!("⚠️ refresh task stopped.");
            Err(anyhow::anyhow!(
                "😵 refresh task stopped unexpectedly. See logs for details."
            ))
        }
        Err(e) => {
            tracing::error!("💥 refresh task panicked : {e}");
            Err(anyhow::anyhow!(
                "💥 refresh task panicked: {e}. See logs for details."
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use sncf::client::ReqwestClient;
    use sncf::{SncfAPIError, fetch_journeys, fetch_places};

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

    #[tokio::test]
    async fn refresh_task_result_ok_becomes_error() {
        let err = refresh_task_result_to_err(Ok(())).unwrap_err();
        assert_eq!(
            err.to_string(),
            "😵 refresh task stopped unexpectedly. See logs for details."
        );
    }

    #[tokio::test]
    async fn refresh_task_result_join_error_panics_message() {
        let handle = tokio::spawn(async { panic!("boom") });
        let join_err = handle.await.unwrap_err();

        let err = refresh_task_result_to_err(Err(join_err)).unwrap_err();
        assert!(
            err.to_string().contains("💥 refresh task panicked:"),
            "unexpected error: {err}"
        );
    }

    #[tokio::test]
    async fn run_returns_ok_after_stop_message() {
        let result = run("change_me".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "hits live SNCF API"]
    // Take care you need to export the SNCF_API_KEY
    async fn test_fetch_places_live_api() {
        let api_key =
            std::env::var("SNCF_API_KEY").expect("set SNCF_API_KEY to run the live API test");
        let client = ReqwestClient::new();

        let results = fetch_places(&client, &api_key, "Grenoble")
            .await
            .expect("expected live SNCF API to return places");

        dbg!(&results);

        assert!(
            !results.is_empty(),
            "expected at least one stop_area place from live API"
        );
        assert!(
            results.iter().any(|place| {
                place.id == "stop_area:SNCF:87747006" && place.name == "Grenoble (Grenoble)"
            }),
            "expected Grenoble (Grenoble) stop_area in live API results"
        );
    }

    #[tokio::test]
    #[ignore = "hits live SNCF API"]
    // Take care you need to export the SNCF_API_KEY
    async fn test_fetch_journeys_live_api() {
        let api_key =
            std::env::var("SNCF_API_KEY").expect("set SNCF_API_KEY to run the live API test");
        let client = ReqwestClient::new();

        // fetch_journeys should return 25 items.
        let results = fetch_journeys(
            &client,
            &api_key,
            "stop_area:SNCF:87747006",
            "stop_area:SNCF:87747337",
        )
        .await
        .expect("expected live SNCF API to return journeys");

        dbg!(&results);

        assert!(
            !results.is_empty(),
            "expected at least one journey from live API"
        );

        assert_eq!(25, results.len());
    }

    #[tokio::test]
    #[ignore = "hits live SNCF API"]
    async fn fetch_places_live_api_invalid_api_key() {
        let client = ReqwestClient::new();
        let err = fetch_places(&client, "invalid_api_key", "Grenoble")
            .await
            .expect_err("expected invalid api key to return an error");

        match err {
            SncfAPIError::ApiError { status, message } => {
                assert!(
                    status == 401 || status == 403,
                    "unexpected status for invalid api key: {status}"
                );
                assert!(
                    message.contains("Token absent"),
                    "unexpected api error message: {message}"
                );
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
