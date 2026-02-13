use std::time::Duration;

use std::env;

use clap::{Parser, Subcommand};
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
// TODO: Fix the command below
enum Commands {
    Places {
        /// Search query. Use quotes for spaces, e.g. "Saint Michel"
        query: String,
    },
    Zourneys {
        /// Start id of the Journeys (stop_area:SNCF:87686006)
        start: String,
        // TODO: run test and fix the help message
        /// Destination of the Journeys (stop_area:SNCF:87747006)
        destination: String,
    },
}

pub async fn run(api_key: String) -> anyhow::Result<()> {
    let cli = Cli::parse();
    run_with_cli_and_limit(api_key, cli, None).await
}

async fn run_with_cli_and_limit(
    api_key: String,
    cli: Cli,
    max_updates: Option<usize>,
) -> anyhow::Result<()> {
    match cli.command {
        Commands::Places { query } => {
            let client = ReqwestClient::new();
            // TODO: Fix the function calling error
            let places = fetch_places(client, api_key, query).await?;

            println!("Places for {query}:");
            for place in places {
                println!("{}\t{}", place.id, place.name);
            }
            return Ok(());
        }

        #[allow(unused_variables)]
        Commands::Journeys { start, destination } => {}
    }

    let (data_sender, mut data_receiver) = mpsc::channel::<String>(5);

    // Spawn a task here that will send data from the API.
    let refresh_task = spawn_refresh_task(data_sender);

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
                tracing::debug!("Data: {data}");
                // For testing purpose, we send a stop message after 5 iterations to go out of the
                // loop.
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

fn spawn_refresh_task(data_sender: mpsc::Sender<String>) -> JoinHandle<()> {
    tokio::spawn(async move {
        tracing::info!("refresh task started");

        loop {
            tracing::info!("sending data");
            if let Err(e) = data_sender.send("Hello".to_string()).await {
                tracing::error!("Error sending message: {e}");
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
        }

        tracing::error!("refresh task terminated");
    })
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;
    use sncf::client::ReqwestClient;
    use sncf::{SncfAPIError, fetch_journeys, fetch_places};
    use tokio::time::timeout;

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
    async fn run_with_cli_and_limit_journeys_exits_on_limit() {
        let cli = Cli {
            command: Commands::Journeys {
                start: "stop_area:SNCF:87747006".to_string(),
                destination: "stop_area:SNCF:87747337".to_string(),
            },
        };

        let res = timeout(
            Duration::from_secs(3),
            run_with_cli_and_limit("unused_api_key".to_string(), cli, Some(1)),
        )
        .await
        .expect("run_with_cli_and_limit should complete before timeout");

        assert!(res.is_ok(), "expected Ok(()), got: {res:?}");
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
    async fn run_with_cli_and_limit_places_grenoble_live_api() {
        let api_key =
            std::env::var("SNCF_API_KEY").expect("set SNCF_API_KEY to run the live API test");
        let cli = Cli {
            command: Commands::Places {
                query: "Grenoble".to_string(),
            },
        };

        run_with_cli_and_limit(api_key, cli, None)
            .await
            .expect("expected places command to succeed for Grenoble");
    }
}
