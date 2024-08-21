use std::collections::HashMap;
use reqwest::StatusCode;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_repr::*;
use url::Url;

use crate::endpoints::light::{Light, LightState};
use crate::endpoints::configuration::ApiToken;
use crate::endpoints::configuration::TokenRequest;


#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum DeconzErrorType {
    UnauthorizedUser = 1,
    InvalidJson = 2,
    ResourceNotAvailable = 3,
    MethodNotAvailable = 4,
    MissingParameter = 5,
    ParameterNotAvailable = 6,
    InvalidValue = 7,
    ParameterNotModifiable = 8,
    TooManyItems = 11,
    DuplicateExist = 100,
    NotAllowedSensorType = 501,
    SensorListFull = 502,
    RuleEngineFull = 601,
    ConditionError = 607,
    ActionError = 608,
    InternalError = 901,
    NotConnected = 950,
    BridgeBusy = 951,
    LinkButtonNotPressed = 101,
    DeviceOff = 201,
    DeviceNotReachable = 202,
    BridgeGroupTableFull = 301,
    DeviceGroupTableFull = 302,
    DeviceScenesTableFull = 402,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestResponse<SuccessType> {
    #[serde(rename(serialize = "error", deserialize = "error"))]
    Error {
        r#type: DeconzErrorType,
        address: String,
        description: String,
    },
    #[serde(rename(serialize = "success", deserialize = "success"))]
    Success(SuccessType),
}

struct Error {
    r#type: DeconzErrorType,
    address: String,
    description: String,
}


#[derive(Debug)]
pub struct DeconzConnection {
    pub url: Url,
    pub api_key: String,
    api_url: Url,
    client: reqwest::Client,
}


impl DeconzConnection {
    pub fn new(url: Url, api_key: String) -> Result<DeconzConnection, Box<dyn std::error::Error>> {
        let api_url = url.join("api/")?.join(&format!("{api_key}/"))?;
        let client = reqwest::ClientBuilder::new()
            .build()
            .expect("Failed to create client.");

        Ok(DeconzConnection {
            url,
            api_key,
            api_url,
            client,
        })
    }

    pub async fn new_without_key(
        url: Url,
        requested_user: TokenRequest,
    ) -> Result<DeconzConnection, Box<dyn std::error::Error>> {
        let api_path = url.join("api").expect("failed to build url");
        let client = reqwest::ClientBuilder::new()
            .build()
            .expect("Failed to create client.");

        let response = match client
            .post(api_path.clone())
            .json(&requested_user)
            .send()
            .await
        {
            Ok(response) => response,
            Err(err) => return Err(format!("Network error {}", err).into()),
        };
        let token_response = match response.json::<Vec<RequestResponse<ApiToken>>>().await {
            Ok(mut res) => {
                let res = res.remove(0);
                match res {
                    RequestResponse::Success(token_response) => { token_response }
                    RequestResponse::Error { address, description, r#type } => {
                        return Err(format!("Api endpoint {address} failed ('{description}', type: {:?})", r#type).into());
                    }
                }
            }
            Err(err) => return Err(format!("Unexpected API response with error {err}").into()),
        };

        let api_key = &token_response.username;
        let api_url = url.join("api/").unwrap().join(api_key).unwrap();

        Ok(DeconzConnection {
            client,
            url,
            api_key: api_key.to_string(),
            api_url,
        })
    }


    pub fn get_api_key(&self) -> &String {
        &self.api_key
    }


    async fn get_request<Response>(&self, url: Url) -> Result<Response, Box<dyn std::error::Error>>
        where
            Response: DeserializeOwned,
    {
        let response = match self.client.get(url).send().await {
            Ok(response) => response,
            Err(err) => return Err(format!("Network error {}", err).into()),
        };
        if response.status() == StatusCode::FORBIDDEN {
            return Err("unauthorized user".to_string().into());
        }
        match response.json::<Response>().await {
            Ok(response) => { return Ok(response); }
            Err(err) => { panic!("Deserializing failed with: {err}") }
        }
    }

    async fn put_request<T, Response>(&self, url: Url, data: T) -> Result<Response, Box<dyn std::error::Error>>
        where
            T: Serialize,
            Response: DeserializeOwned,
    {
        let response = match self.client.put(url).json(&data).send().await {
            Ok(response) => response,
            Err(err) => return Err(format!("Network error {}", err).into()),
        };
        match response.json::<Response>().await {
            Ok(response) => { Ok(response) }
            Err(err) => { panic!("Deserializing failed with error: {err}") }
        }
    }

    pub async fn get_all_lights(&self) -> Result<HashMap<String, Light>, Box<dyn std::error::Error>> {
        let url = self.api_url.join("lights").unwrap();
        self.get_request(url).await
    }

    pub async fn get_light_state(&self, id: &str) -> Result<LightState, Box<dyn std::error::Error>> {
        let url = self
            .api_url
            .join("lights/").expect("failed to build url")
            .join(id).expect("failed to build url");
        self.get_request(url).await
    }

    pub async fn set_light_state(
        &self,
        id: &str,
        new_state: &LightState,
    ) -> Result<Vec<RequestResponse<HashMap<String, serde_json::Value>>>, Box<dyn std::error::Error>> {
        let url = self
            .api_url
            .join("lights/").expect("failed to build url")
            .join(format!("{id}/").as_str()).expect("failed to build url")
            .join("state").expect("failed to build url");

        self.put_request(url, new_state).await
    }

    pub async fn set_light_attributes(
        &self,
        id: &str,
        attrs: &LightAttributes,
    ) -> Result<Vec<RequestResponse<HashMap<String, serde_json::Value>>>, Box<dyn std::error::Error>> {
        let url = self
            .api_url
            .join("lights/").expect("failed to build url")
            .join(format!("{id}").as_str()).expect("failed to build url");
        self.put_request(url, attrs).await
    }
}

#[derive(Serialize, Deserialize)]
pub struct LightAttributes {
    name: String,
}


