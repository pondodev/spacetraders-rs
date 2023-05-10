use curl::easy::Easy;
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
}

impl API {
    pub fn new(mock: bool) -> Self {
        return API {
            base_url: if mock { MOCK_BASE_URL.to_owned() } else { BASE_URL.to_owned() },
        }
    }

    fn do_request(mut handle: Easy, body: Option<serde_json::Value>, full_route: String) -> Result<Response, String> {
        // TODO: write body to request
        match body {
            Some(json) => { /* TODO */ },
            None => {},
        }

        let mut response: Vec<u8> = Vec::new();
        let mut transfer = handle.transfer();
        let res = transfer.write_function(|data| {
            response.extend_from_slice(data);
            Ok(data.len())
        });

        match res {
            Ok(_)  => {},
            Err(e) => return Err(format!("API.get('{}'): failed to set write function: {}", full_route, e.description())),
        }

        let res = transfer.perform();
        match res {
            Ok(_)  => {},
            Err(e) => return Err(format!("[{}] API.get('{}'): {}", e.code(), full_route, e.description())),
        }

        drop(transfer);
        drop(handle);

        // TODO: actually read response type
        return Ok(Response { body_type: ResponseBodyType::JSON, body: response })
    }

    fn get(&self, route: &str) -> Result<Response, String> {
        let full_route = format!("{}{}", self.base_url, route);
        let mut handle = Easy::new();

        let res = handle.url(&full_route);
        match res {
            Ok(_)  => {},
            Err(e) => return Err(format!("API.get('{}'): failed to set url: {}", full_route, e.description())),
        }

        let res = handle.get(true);
        match res {
            Ok(_)   => {},
            Err(e) => return Err(format!("API.get({}): failed to set request to GET: {}", full_route, e.to_string())),
        }

        return API::do_request(handle, None, full_route)
    }

    fn delete(&self, route: &str) -> Result<Response, String> {
        // TODO: implement
        return Err("TODO".to_owned())
    }

    fn post(&self, route: &str, body: Option<serde_json::Value>) -> Result<Response, String> {
        // TODO: implement
        return Err("TODO".to_owned())
    }

    fn put(&self, route: &str, body: Option<serde_json::Value>) -> Result<Response, String> {
        // TODO: implement
        return Err("TODO".to_owned())
    }

    pub fn factions(&self) -> Result<factions::list::Response, String> {
        let response = self.get("/factions")?;

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
}

