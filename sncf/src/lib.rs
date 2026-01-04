pub mod client;
mod fake;

use jiff::tz::TimeZone;
use jiff::{Unit, Zoned, civil};
use thiserror::Error;

use crate::client::HTTPClient;

#[derive(Error, Debug)]
pub enum SncfAPIError {
    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),
    #[error("Deserialization failed: {0}")]
    Deserialization(#[from] serde_json::Error),
    #[error("Date cannot be parsed: {0}")]
    DateParse(String),
    #[error("Invalid duration: {0}")]
    InvalidDuration(String),
    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Place {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub embedded_type: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlacesResponse {
    pub places: Vec<Place>,
}

#[derive(Debug, Clone)]
pub struct Journey {
    pub dep: Zoned,
    pub arr: Zoned,
    pub date_str: String,
    pub duration_secs: i64,
    pub nb_transfers: i64,
}

#[derive(Debug, serde::Deserialize)]
struct JourneysResponse {
    #[serde(default)]
    journeys: Vec<JourneyItem>,
}
#[derive(Debug, serde::Deserialize)]
struct JourneyItem {
    #[serde(default)]
    departure_date_time: String,
    #[serde(default)]
    arrival_date_time: String,
    #[serde(default)]
    nb_transfers: Option<i64>,
}

/// Fetch places matching a query and keep only `stop_area` results.
///
/// Uses the SNCF API via an [`HTTPClient`]. The returned list only contains
/// entries whose `embedded_type` is `"stop_area"`.
///
/// # Example
///
/// ```rust,no_run
/// use sncf::{client::ReqwestClient, fetch_places};
///
/// # async fn run() -> Result<(), sncf::SncfAPIError> {
/// let client = ReqwestClient::new();
/// let results = fetch_places(&client, "your_api_key", "Grenoble").await?;
///
/// assert!(results.iter().all(|p| p.embedded_type.as_deref() == Some("stop_area")));
/// Ok(())
/// # }
/// ```
pub async fn fetch_places(
    client: &impl HTTPClient,
    api_key: &str,
    query: &str,
) -> Result<Vec<Place>, SncfAPIError> {
    let url = format!(
        "https://api.sncf.com/v1/coverage/sncf/places?q={}",
        urlencoding::encode(query)
    );
    let parsed: PlacesResponse = client.get(&url, api_key, Some("")).await?;
    Ok(parsed
        .places
        .into_iter()
        .filter(|p| matches!(p.embedded_type.as_deref(), Some("stop_area")))
        .collect())
}

/// Fetch journeys between two place identifiers.
///
/// This calls the SNCF journeys endpoint and parses the response into
/// [`Journey`] values with computed duration and formatted date.
///
/// # Example
///
/// ```rust,no_run
/// use sncf::{client::ReqwestClient, fetch_journeys};
///
/// # async fn run() -> Result<(), sncf::SncfAPIError> {
/// let client = ReqwestClient::new();
/// let journeys = fetch_journeys(
///     &client,
///     "your_api_key",
///     "stop_area:SNCF:87747006",
///     "stop_area:SNCF:87747337",
/// )
/// .await?;
///
/// assert!(!journeys.is_empty());
/// Ok(())
/// # }
/// ```
pub async fn fetch_journeys(
    client: &impl HTTPClient,
    api_key: &str,
    from_id: &str,
    to_id: &str,
) -> Result<Vec<Journey>, SncfAPIError> {
    let url = build_journeys_url(from_id, to_id);
    let parsed: JourneysResponse = client.get(&url, api_key, Some("")).await?;
    parsed
        .journeys
        .into_iter()
        .map(|j| {
            let dep = parse_sncf_dt(&j.departure_date_time)?;
            let arr = parse_sncf_dt(&j.arrival_date_time)?;
            let date_str = format_date(&dep);
            let dur = &arr - &dep;
            let duration_secs = dur.total(Unit::Second).map_err(|_| {
                SncfAPIError::InvalidDuration(format!(
                    "Invalid duration from {} to {}",
                    j.departure_date_time, j.arrival_date_time
                ))
            })?;
            Ok(Journey {
                dep,
                arr,
                date_str,
                duration_secs: duration_secs as i64,
                nb_transfers: j.nb_transfers.unwrap_or(0),
            })
        })
        .collect()
}

fn build_journeys_url(from_id: &str, to_id: &str) -> String {
    let base = "https://api.sncf.com/v1/coverage/sncf/journeys";
    let from = urlencoding::encode(from_id);
    let to = urlencoding::encode(to_id);
    format!(
        "{base}?from={from}&to={to}&first_section_mode[]=walking&last_section_mode[]=walking&min_nb_transfers=0&direct_path=none&min_nb_journeys=25&is_journey_schedules=True"
    )
}

pub fn parse_sncf_dt(s: &str) -> Result<Zoned, SncfAPIError> {
    if s.len() < 15 {
        return Err(SncfAPIError::DateParse(s.to_string()));
    }
    let y = s[0..4]
        .parse()
        .map_err(|_| SncfAPIError::DateParse(s.to_string()))?;
    let m = s[4..6]
        .parse()
        .map_err(|_| SncfAPIError::DateParse(s.to_string()))?;
    let d = s[6..8]
        .parse()
        .map_err(|_| SncfAPIError::DateParse(s.to_string()))?;
    let hh = s[9..11]
        .parse()
        .map_err(|_| SncfAPIError::DateParse(s.to_string()))?;
    let mm = s[11..13]
        .parse()
        .map_err(|_| SncfAPIError::DateParse(s.to_string()))?;
    let ss = s[13..15]
        .parse()
        .map_err(|_| SncfAPIError::DateParse(s.to_string()))?;
    let dt = civil::date(y, m, d).at(hh, mm, ss, 0);
    let tz = TimeZone::get("Europe/Paris").unwrap();
    match tz.to_zoned(dt) {
        Ok(zone) => Ok(zone),
        Err(_) => Err(SncfAPIError::DateParse(s.to_string())),
    }
}

pub fn format_hm(z: &Zoned) -> String {
    let s = z.to_string();
    if s.len() >= 16 {
        s[11..16].to_string()
    } else {
        s
    }
}
pub fn format_date(z: &Zoned) -> String {
    let s = z.to_string();
    if s.len() >= 10 {
        s[0..10].to_string()
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::client::FakeClient;

    use super::*;

    #[tokio::test]
    async fn test_fetch_places_filters_stop_area() {
        let client = FakeClient::new();
        let results = fetch_places(&client, "key", "Grenoble").await.unwrap();

        assert_eq!(results.len(), 8, "only stop_area should remain");
        assert_eq!(results[0].id, "stop_area:SNCF:87747006");
        assert_eq!(results[7].id, "stop_area:SNCF:87335521");
    }

    #[tokio::test]
    async fn test_fetch_journeys() {
        let client = FakeClient::new();
        let results = fetch_journeys(
            &client,
            "key",
            "stop_area:SNCF:87747006",
            "stop_area:SNCF:87747337",
        )
        .await;
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 2);

        let first = &results[0];
        assert_eq!(first.nb_transfers, 1);
        assert_eq!(format_hm(&first.dep), "06:21");
        assert_eq!(format_hm(&first.arr), "06:49");
        assert_eq!(first.date_str, "2026-01-03");
        assert_eq!(first.duration_secs, 1680);

        let second = &results[1];
        assert_eq!(second.nb_transfers, 0);
        assert_eq!(format_hm(&second.dep), "07:54");
        assert_eq!(format_hm(&second.arr), "08:03");
        assert_eq!(second.date_str, "2026-01-03");
        assert_eq!(second.duration_secs, 540);
    }

    #[tokio::test]
    async fn test_fetch_journeys_invalid_date() {
        let client = FakeClient::new();
        // The stop_area: stop_area:SNCF:87747338 which is returning an invalid date.
        let results = fetch_journeys(
            &client,
            "key",
            "stop_area:SNCF:87747006",
            "stop_area:SNCF:87747338",
        )
        .await;
        assert!(results.is_err());
        let err = results.unwrap_err();
        assert!(matches!(
            err,
            SncfAPIError::DateParse(ref value) if value == "I'm invalid"
        ));
        assert_eq!(err.to_string(), "Date cannot be parsed: I'm invalid");
    }
}
