use crate::Configuration;
use crate::Error;
use crate::Result;
use reqwest::header::HeaderMap;
use reqwest::Response;
use reqwest::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use serde_json as json;
use serde_json::Value;

pub trait Resource {
    fn id(&self) -> Option<&str>;
    fn member(&self) -> &str;
    fn collection(&self) -> &str;
}

#[derive(Clone, Default)]
pub struct Adapter {
    config: Configuration,
    client: reqwest::Client,
}

impl Adapter {
    pub fn new(config: Configuration) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub async fn call<T>(&self, method: &str, resource: &T) -> Result<Option<T>>
    where
        T: Resource + Serialize + for<'de> Deserialize<'de>,
    {
        let response = self.send_request(method, resource).await?;

        match response.status() {
            StatusCode::NO_CONTENT => Ok(None),
            StatusCode::CREATED => Ok(Some(self.extract_attributes(response, resource).await?)),
            _ => Err(self.extract_errors(response).await?),
        }
    }

    fn build_delete_request<T>(&self, uri: &str, _resource: &T) -> Result<reqwest::Request> {
        self.client
            .delete(uri)
            .headers(self.headers())
            .build()
            .map_err(|e| e.into())
    }

    fn build_get_request<T>(&self, uri: &str, _resource: &T) -> Result<reqwest::Request> {
        self.client
            .get(uri)
            .headers(self.headers())
            .build()
            .map_err(|e| e.into())
    }

    fn build_post_request<T>(&self, uri: &str, resource: &T) -> Result<reqwest::Request>
    where
        T: Serialize,
    {
        self.client
            .post(uri)
            .headers(self.headers())
            .json(&resource)
            .build()
            .map_err(|e| e.into())
    }

    async fn extract_attributes<T>(&self, response: Response, resource: &T) -> Result<T>
    where
        T: Resource + for<'de> Deserialize<'de>,
    {
        let member = resource.member();
        let value: Value = json::from_slice(&response.bytes().await?)?;
        let value: Value = value.get(member).unwrap().to_owned();
        Ok(json::from_value(value)?)
    }

    async fn extract_errors(&self, response: Response) -> Result<Error> {
        let payload: Value = json::from_slice(&response.bytes().await?)?;
        let errors: Vec<Value> = json::from_value(payload["errors"].to_owned())?;
        let errors: Vec<String> = errors
            .iter()
            .map(|error| error["detail"].to_string())
            .collect();

        Ok(Error {
            errors,
            status: "error".into(),
        })
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.config.private_key)
                .parse()
                .unwrap(),
        );
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("User-Agent", "Rust".parse().unwrap());
        headers
    }

    async fn send_request<T>(&self, method: &str, resource: &T) -> Result<Response>
    where
        T: Resource + Serialize,
    {
        let uri = self.url_for(resource);
        let request = match method {
            "get" => self.build_get_request(&uri, &resource)?,
            "post" => self.build_post_request(&uri, &resource)?,
            "delete" => self.build_delete_request(&uri, &resource)?,
            _ => panic!(),
        };
        self.client.execute(request).await.map_err(|e| e.into())
    }

    fn url_for<T>(&self, resource: &T) -> String
    where
        T: Resource,
    {
        let collection = resource.collection();
        let mut endpoint = format!(
            "{}/{}/{}",
            self.config.host, self.config.namespace, collection
        );
        if let Some(resource_id) = resource.id() {
            endpoint += &format!("/{}", resource_id)
        };
        endpoint
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self {
            errors: vec![e.to_string()],
            status: "error".into(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self {
            errors: vec![e.to_string()],
            status: "error".into(),
        }
    }
}
