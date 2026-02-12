use std::env;
use std::time::Duration;

use anyhow::Context;
use clap::{Parser, Subcommand};
use jiff::{Unit, Zoned};
use sncf::{Journey, SncfAPIError, fetch_journeys};
use sncf::{client::ReqwestClient, fetch_places};
use tokio::sync::mpsc::{self, error};
use tokio::task::JoinHandle;

pub const APPNAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser, Debug)]
#[command(name = APPNAME, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Places {
        /// Search query. Use quotes for spaces, e.g. "Saint Michel"
        query: String,
    },
    Journeys {
        /// Start id of the Journeys (stop_area:SNCF:87686006)
        start: String,
        /// Destination id of the Journeys (stop_area:SNCF:87747006)
        destination: String,
    },
}

#[allow(unused)]
pub async fn run(api_key: String) -> anyhow::Result<()> {
    let cli = Cli::parse();
    run_with_cli_and_limit(api_key, cli, None).await
}

async fn run_with_cli_and_limit(
    api_key: String,
    cli: Cli,
    max_updates: Option<usize>,
) -> anyhow::Result<()> {
    let (start, destination) = match cli.command {
        Commands::Places { query } => {
            let client = ReqwestClient::new();
            let places = fetch_places(&client, &api_key, &query).await?;

            println!("Places for {query}:");
            for place in places {
                println!("{}\t{}", place.id, place.name);
            }
            return Ok(());
        }

        Commands::Journeys { start, destination } => (start, destination),
    };

    let (data_sender, mut data_receiver) = mpsc::channel::<Result<Vec<Journey>, SncfAPIError>>(5);

    // Spawn a task here that will send data from the API.
    let start_label = start.clone();
    let destination_label = destination.clone();
    let refresh_task = spawn_refresh_task(data_sender, api_key, start, destination);

    // Following loop simulate the main loop.
    let mut updates = 0usize;
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
                tracing::debug!("Data: {data:?}");
                let update_time = format_update_time();
                println!(
                    "===================================================================================================================================="
                );
                println!(
                    "{}  -> {}             Update time:{}",
                    start_label, destination_label, update_time
                );
                println!(
                    "------------------------------------------------------------------------------------------------------------------------------------"
                );
                let mut journeys = data.context("Fail to retrieve journeys")?;
                if journeys.is_empty() {
                    println!("No journeys");
                } else {
                    journeys.sort_by_key(|j| j.dep.clone());
                    let now = Zoned::now().round(Unit::Second);
                    for journey in &journeys {
                        let dur_min = journey.duration_secs / 60;
                        let remaining_min = match &now {
                            Ok(now) => {
                                let diff = &journey.dep - now;
                                diff.total(Unit::Minute).unwrap_or(0.0) as i64
                            }
                            Err(_) => 0,
                        };
                        let dep_str = sncf::format_hm(&journey.dep);
                        println!(
                            "Date: {:<10}  Dur: {:<3}  Changes: {:<2}  Dep at: {:<5}  In: {}m",
                            journey.date_str,
                            format!("{dur_min}m"),
                            journey.nb_transfers,
                            dep_str,
                            remaining_min,
                        );
                    }
                }
                println!(
                    "====================================================================================================================================\n"
                );
                updates += 1;
                if let Some(limit) = max_updates
                    && updates >= limit
                {
                    break;
                }
            }
        };
        tokio::time::sleep(Duration::from_millis(100)).await
    }
    Ok(())
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

fn format_update_time() -> String {
    let now = Zoned::now()
        .round(Unit::Second)
        .map(|z| z.to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    if now.len() >= 19 {
        now[11..19].to_string()
    } else {
        now
    }
}

fn spawn_refresh_task(
    data_sender: mpsc::Sender<Result<Vec<Journey>, SncfAPIError>>,
    api_key: String,
    start_id: String,
    destination_id: String,
) -> JoinHandle<()> {
    let client = ReqwestClient::new();
    tokio::spawn(async move {
        tracing::info!("refresh task started");

        loop {
            tracing::info!("sending data");
            let msg = fetch_journeys(&client, &api_key, &start_id, &destination_id).await;
            if let Err(e) = data_sender.send(msg).await {
                tracing::error!("Error sending message: {e}");
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }

        tracing::error!("refresh task terminated");
    })
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;
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
    fn help_contains_expected_sections() {
        let mut root_cmd = Cli::command();
        let mut root_buf = Vec::new();
        root_cmd.write_long_help(&mut root_buf).unwrap();
        let root_help = String::from_utf8(root_buf).unwrap();

        assert!(root_help.contains("Usage:"));
        assert!(root_help.contains("places"));
        assert!(root_help.contains("journeys"));

        let mut journeys_cmd = Cli::command()
            .find_subcommand_mut("journeys")
            .expect("journeys subcommand should exist")
            .clone();
        let mut journeys_buf = Vec::new();
        journeys_cmd.write_long_help(&mut journeys_buf).unwrap();
        let journeys_help = String::from_utf8(journeys_buf).unwrap();

        assert!(journeys_help.contains("Start id of the Journeys (stop_area:SNCF:87686006)"));
        assert!(journeys_help.contains("Destination id of the Journeys (stop_area:SNCF:87747006)"));
    }

    #[test]
    fn cli_parse_places_command_ok() {
        let cli = Cli::try_parse_from(["async_rust_cli", "places", "Grenoble"])
            .expect("places command should parse");
        assert!(matches!(
            cli.command,
            Commands::Places { query } if query == "Grenoble"
        ));
    }

    #[test]
    fn cli_parse_journeys_command_ok() {
        let cli = Cli::try_parse_from([
            "async_rust_cli",
            "journeys",
            "stop_area:SNCF:87747006",
            "stop_area:SNCF:87747337",
        ])
        .expect("journeys command should parse");
        assert!(matches!(
            cli.command,
            Commands::Journeys { start, destination }
            if start == "stop_area:SNCF:87747006" && destination == "stop_area:SNCF:87747337"
        ));
    }

    #[test]
    fn cli_parse_journeys_missing_destination_fails() {
        let err = Cli::try_parse_from(["async_rust_cli", "journeys", "stop_area:SNCF:87747006"])
            .expect_err("missing destination should fail");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn cli_parse_missing_subcommand_fails() {
        let err =
            Cli::try_parse_from(["async_rust_cli"]).expect_err("missing subcommand should fail");
        assert_eq!(
            err.kind(),
            clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
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
        let first = &results[0];
        assert!(
            first.duration_secs > 0,
            "expected positive duration for first journey"
        );
        assert!(
            first.dep <= first.arr,
            "expected departure to be before or equal to arrival"
        );
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

    #[tokio::test]
    async fn run_with_args_ok_ids_returns_ok() {
        let api_key =
            std::env::var("SNCF_API_KEY").expect("set SNCF_API_KEY to run the live API test");

        let cli = Cli::try_parse_from([
            "async_rust_cli",
            "journeys",
            "stop_area:SNCF:87747006",
            "stop_area:SNCF:87747337",
        ])
        .expect("valid CLI args");

        let res = run_with_cli_and_limit(api_key, cli, Some(1)).await;
        assert!(res.is_ok(), "expected Ok, got: {res:?}");
    }

    #[tokio::test]
    async fn run_with_args_bad_ids_returns_err() {
        let api_key =
            std::env::var("SNCF_API_KEY").expect("set SNCF_API_KEY to run the live API test");

        let cli = Cli::try_parse_from([
            "async_rust_cli",
            "journeys",
            "stop_area:SNCF:00000000",
            "stop_area:SNCF:00000001",
        ])
        .expect("valid CLI args");

        let res = run_with_cli_and_limit(api_key, cli, Some(1)).await;
        let err = res.expect_err("expected Err for bad ids");
        let msg = err.to_string();
        assert!(
            msg.contains("Fail to retrieve journeys"),
            "expected Fail to retrieve journeys error, got: {msg}"
        );
        assert!(
            !msg.contains("panicked"),
            "expected no panic-based error, got: {msg}"
        );
    }
}
