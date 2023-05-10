use reqwest::{Client, RequestBuilder, header};
use serde_json;

use crate::api_types::factions;

const BASE_URL: &str = "https://api.spacetraders.io/v2";
const MOCK_BASE_URL: &str = "https://stoplight.io/mocks/spacetraders/spacetraders/96627693";

#[allow(dead_code)]

#[derive(Debug)]
enum ResponseBodyType {
    None,
    JSON,
}

struct Response {
    body_type: ResponseBodyType,
    body: Vec<u8>,
}

pub struct API {
    base_url: String,
    auth_token: String,
    client: Client,
}

impl API {
    pub fn new(mock: bool) -> Self {
        return API {
            base_url: if mock { MOCK_BASE_URL.to_owned() } else { BASE_URL.to_owned() },
            auth_token: "TODO".to_owned(),
            client: Client::new(),
        }
    }

    async fn do_request(&self, mut request: RequestBuilder) -> Result<Response, String> {
        request = request.bearer_auth(&self.auth_token);
        let response = request.send().await;

        match response {
            Ok(r)  => {
                let header = r.headers();
                let content_type = match header.get(header::CONTENT_TYPE) {
                    Some(v) => {
                        println!("got content-type {:?}", v);
                        ResponseBodyType::JSON
                    },
                    None    => ResponseBodyType::None,
                };

                let response_body: Vec<u8> = match r.bytes().await {
                    Ok(b)  => Vec::from(b),
                    Err(_) => Vec::new(),
                };
                return Ok(Response { body_type: content_type, body: response_body })
            },
            Err(e) => return Err(format!("failed to make request: {}", e.to_string())),
        }
    }

    async fn get(&self, route: String) -> Result<Response, String> {
        let full_route = format!("{}{}", self.base_url, route);
        let request = self.client.get(&full_route);

        return self.do_request(request).await
    }

    async fn post(&self, route: &str, body: Option<serde_json::Value>) -> Result<Response, String> {
        // TODO: implement
        return Err("TODO".to_owned())
    }

    async fn patch(&self, route: &str, body: Option<serde_json::Value>) -> Result<Response, String> {
        // TODO: implement
        return Err("TODO".to_owned())
    }

    pub async fn list_factions(&self) -> Result<factions::list::Response, String> {
        let response = self.get("/factions".to_owned()).await?;

        return match response.body_type {
            ResponseBodyType::JSON => {
                let json = serde_json::from_slice::<factions::list::Response>(&response.body);
                match json {
                    Ok(val) => Ok(val),
                    Err(e)  => Err(format!("failed to parse json: {}", e.to_string())),
                }
            },
            _ => Err(format!("got invalid response body type ({:?})", response.body_type)),
        }
    }

    pub async fn get_faction(&self, faction_symbol: &str) -> Result<factions::get::Response, String> {
        let response = self.get(format!("/factions/{}", faction_symbol)).await?;

        return match response.body_type {
            ResponseBodyType::JSON => {
                let json = serde_json::from_slice::<factions::get::Response>(&response.body);
                match json {
                    Ok(val) => Ok(val),
                    Err(e)  => Err(format!("failed to parse json: {}", e.to_string())),
                }
            },
            _ => Err(format!("got invalid response body type ({:?})", response.body_type)),
        }
    }
}

