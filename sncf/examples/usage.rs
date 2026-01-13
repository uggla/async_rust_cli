use sncf::{
    client::ReqwestClient, fetch_journeys, fetch_places, format_hm, SncfAPIError,
};

#[tokio::main]
async fn main() -> Result<(), SncfAPIError> {
    let api_key = std::env::var("SNCF_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        eprintln!("Missing SNCF_API_KEY env var.");
        std::process::exit(1);
    }

    let client = ReqwestClient::new();

    let places = fetch_places(&client, &api_key, "Grenoble").await?;
    println!("Places for Grenoble:");
    for place in &places {
        println!("- {} ({})", place.name, place.id);
    }

    let from_id = "stop_area:SNCF:87747006"; // Grenoble
    let to_id = "stop_area:SNCF:87747337"; // Voreppe
    let journeys = fetch_journeys(&client, &api_key, from_id, to_id).await?;
    println!("\nJourneys Grenoble -> Voreppe:");
    for journey in journeys.iter().filter(|journey| journey.nb_transfers == 0) {
        println!(
            "- {} {} -> {} ({} min, {} transfers)",
            journey.date_str,
            format_hm(&journey.dep),
            format_hm(&journey.arr),
            journey.duration_secs,
            journey.nb_transfers
        );
    }

    Ok(())
}
