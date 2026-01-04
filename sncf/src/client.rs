use crate::{SncfAPIError, fake};
use reqwest::Client;
use serde::de::DeserializeOwned;

const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (rust; tokio; +https://github.com/uggla/async_rust_tui)"
);

pub trait HTTPClient {
    fn get<T: DeserializeOwned + Send>(
        &self,
        url: &str,
        username: &str,
        password: Option<&str>,
    ) -> impl std::future::Future<Output = Result<T, SncfAPIError>> + Send;
}

#[derive(Debug)]
pub struct ReqwestClient(Client);

#[derive(Debug)]
pub(crate) struct FakeClient;

impl ReqwestClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("Fail to initialize client.");
        Self(client)
    }
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HTTPClient for ReqwestClient {
    async fn get<T: DeserializeOwned + Send>(
        &self,
        url: &str,
        username: &str,
        password: Option<&str>,
    ) -> Result<T, SncfAPIError> {
        let response = self
            .0
            .get(url)
            .basic_auth(username, password)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(SncfAPIError::ApiError {
                status: status.as_u16(),
                message: body,
            });
        }

        let res = response.json::<T>().await?;
        Ok(res)
    }
}

#[allow(dead_code)]
impl FakeClient {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FakeClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HTTPClient for FakeClient {
    async fn get<T: DeserializeOwned + Send>(
        &self,
        url: &str,
        _username: &str,
        _password: Option<&str>,
    ) -> Result<T, SncfAPIError> {
        match url {
            "https://api.sncf.com/v1" => Ok(serde_json::from_str(&fake::api_base()).unwrap()),
            "https://api.sncf.com/v1/coverage/sncf/places?q=Grenoble" => {
                Ok(serde_json::from_str(&fake::places()).unwrap())
            }
            "https://api.sncf.com/v1/coverage/sncf/journeys?from=stop_area%3ASNCF%3A87747006&to=stop_area%3ASNCF%3A87747337&first_section_mode[]=walking&last_section_mode[]=walking&min_nb_transfers=0&direct_path=none&min_nb_journeys=25&is_journey_schedules=True" => {
                Ok(serde_json::from_str(&fake::journeys()).unwrap())
            }
            "https://api.sncf.com/v1/coverage/sncf/journeys?from=stop_area%3ASNCF%3A87747006&to=stop_area%3ASNCF%3A87747338&first_section_mode[]=walking&last_section_mode[]=walking&min_nb_transfers=0&direct_path=none&min_nb_journeys=25&is_journey_schedules=True" => {
                Ok(serde_json::from_str(&fake::journeys_invalid_date()).unwrap())
            }
            _ => Ok(serde_json::from_str("{}").unwrap()),
        }
    }
}

// --- Test Module ---
#[cfg(test)]
mod tests {

    use crate::{JourneysResponse, PlacesResponse};
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize)]
    struct ApiBaseResponse {
        links: Vec<ApiLink>,
    }

    #[derive(Debug, Deserialize)]
    struct ApiLink {
        href: String,
        templated: bool,
        rel: String,
        #[serde(rename = "type")]
        link_type: String,
        title: String,
    }

    #[test]
    fn test_user_agent_format() {
        let expected = format!(
            "{}/{} (rust; tokio; +https://github.com/uggla/async_rust_tui)",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        assert_eq!(USER_AGENT, expected);
    }

    #[tokio::test]
    async fn test_get_invalid_url_error() {
        let client = ReqwestClient::new();
        let invalid_url = "this is not a valid url";

        let result: Result<PlacesResponse, SncfAPIError> =
            client.get(invalid_url, "user", None).await;

        // assert that the result is an err
        assert!(result.is_err());

        let error = format!("{:?}", result.unwrap_err());
        assert_eq!(
            "HttpRequest(reqwest::Error { kind: Builder, source: RelativeUrlWithoutBase })",
            &error
        );
    }

    #[tokio::test]
    async fn test_get_url_not_exist_error() {
        let client = ReqwestClient::new();
        let invalid_url = "http://thisdomaindoesnotexist";

        let result: Result<PlacesResponse, SncfAPIError> =
            client.get(invalid_url, "user", None).await;

        // assert that the result is an err
        assert!(result.is_err());

        let error = format!("{:?}", result.unwrap_err());
        assert_eq!(
            "HttpRequest(reqwest::Error { kind: Request, url: \"http://thisdomaindoesnotexist/\", source: hyper_util::client::legacy::Error(Connect, ConnectError(\"dns error\", Custom { kind: Uncategorized, error: \"failed to lookup address information: Name or service not known\" })) })",
            &error
        );
    }

    #[tokio::test]
    async fn test_fake_client_places() {
        let client = FakeClient::new();
        let url = "https://api.sncf.com/v1/coverage/sncf/places?q=Grenoble";

        let result: Result<PlacesResponse, SncfAPIError> = client.get(url, "user", None).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.places.len(), 9);
        let expected_names = [
            "Grenoble (38000-38100)",
            "Grenoble (Grenoble)",
            "Grenoble Universités - Gières (Gières)",
            "Grenoble - Champollion (Grenoble)",
            "Grenoble Europole (Grenoble)",
            "Grenoble - Eugène Chavant (Grenoble)",
            "Grenoble Cité Internationale (Grenoble)",
            "Grenoble Victor Hugo (Grenoble)",
            "Grenoble - gare Routière (Grenoble)",
        ];
        for (place, expected_name) in response.places.iter().zip(expected_names.iter()) {
            assert_eq!(&place.name, expected_name);
        }
    }

    #[tokio::test]
    async fn test_fake_client_api_base() {
        let client = FakeClient::new();
        let url = "https://api.sncf.com/v1";

        let result: Result<ApiBaseResponse, SncfAPIError> = client.get(url, "user", None).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.links.len(), 4);
        assert_eq!(response.links[0].href, "https://api.sncf.com/v1/coverage/");
        assert!(!response.links[0].templated);
        assert_eq!(response.links[0].rel, "coverage");
        assert_eq!(response.links[0].link_type, "coverage");
        assert_eq!(response.links[0].title, "Coverage of navitia");
    }

    #[tokio::test]
    async fn test_fetch_journeys_client_parsing() {
        let client = FakeClient::new();
        let url = "https://api.sncf.com/v1/coverage/sncf/journeys?from=stop_area%3ASNCF%3A87747006&to=stop_area%3ASNCF%3A87747337&first_section_mode[]=walking&last_section_mode[]=walking&min_nb_transfers=0&direct_path=none&min_nb_journeys=25&is_journey_schedules=True";

        let result: Result<JourneysResponse, SncfAPIError> = client.get(url, "user", None).await;

        assert!(result.is_ok());
        let response = result.unwrap().journeys;
        assert_eq!(response.len(), 2);

        let first = &response[0];
        assert_eq!(first.nb_transfers, Some(1));
        assert_eq!(first.departure_date_time, "20260103T062100");
        assert_eq!(first.arrival_date_time, "20260103T064900");

        let second = &response[1];
        assert_eq!(second.nb_transfers, Some(0));
        assert_eq!(second.departure_date_time, "20260103T075400");
        assert_eq!(second.arrival_date_time, "20260103T080300");
    }

    #[tokio::test]
    async fn test_fetch_journeys_client_sending_invalid_date() {
        let client = FakeClient::new();
        // The url has the stop_area: stop_area:SNCF:87747338 which is returning an invalid date.
        let url = "https://api.sncf.com/v1/coverage/sncf/journeys?from=stop_area%3ASNCF%3A87747006&to=stop_area%3ASNCF%3A87747338&first_section_mode[]=walking&last_section_mode[]=walking&min_nb_transfers=0&direct_path=none&min_nb_journeys=25&is_journey_schedules=True";

        let result: Result<JourneysResponse, SncfAPIError> = client.get(url, "user", None).await;

        // The HTTP client only deserializes JSON; date parsing happens in fetch_journeys.
        // So that should not fail here.
        assert!(result.is_ok());
        let response = result.unwrap().journeys;
        assert_eq!(response.len(), 2);

        let first = &response[0];
        assert_eq!(first.nb_transfers, Some(1));
        // Note the diff ----------------------- --v
        assert_eq!(first.departure_date_time, "I'm invalid");
        assert_eq!(first.arrival_date_time, "20260103T064900");

        let second = &response[1];
        assert_eq!(second.nb_transfers, Some(0));
        assert_eq!(second.departure_date_time, "20260103T075400");
        assert_eq!(second.arrival_date_time, "20260103T080300");
    }
}
