#![allow(dead_code)]
use gloo::net::http::Request;
use yew::platform::spawn_local;
use serde::de::DeserializeOwned;
use anyhow::Error;
use serde::Serialize;
use shared::requests::ResponseStatus;

enum RequestType {
    GET,
    POST,
}

pub struct Fetch {
    request_type: RequestType,
    url: Option<String>,
    data: Option<String>,
}

#[derive(Debug)]
pub enum Status {
    None,
    Error(String),
    OK
}

#[derive(Debug)]
pub struct FetchResponse<T>
{
    pub payload: Option<T>,
    pub status: Status,
}


impl <T>  ResponseStatus for FetchResponse<T>
where T: ResponseStatus
{
    fn is_ok(&self) -> bool {
        if let Some(pl) = &self.payload {
            return pl.is_ok();
        }
        false
    }

    fn get_message(&self) -> Option<String> {
        if let Some(pl) = &self.payload {
            return pl.get_message();
        }
        None
    }

}

impl<T> FetchResponse<T> {
    pub fn empty(status: Status) -> Self {
        Self {
            payload: None,
            status,
        }
    }

    pub fn error(err: &str) -> Self {
        Self {
            payload: None,
            status: Status::Error(err.into()),
        }
    }
}

impl Fetch {
    pub fn new() -> Self {
        Self {
            request_type: RequestType::GET,
            url: None,
            data: None,
        }
    }

    pub fn data<D>(mut self, data: &D) -> Self
    where
        D: Serialize,
    {
        self.data = Some(serde_json::to_string(data).unwrap());
        self
    }

    pub fn post(mut self) -> Self {
        self.request_type = RequestType::POST;
        self
    }

    pub fn get(mut self) -> Self {
        self.request_type = RequestType::GET;
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    async fn send_internal<T>(self) -> Result<FetchResponse<T>, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let request = match (self.request_type, self.url) {
            (RequestType::GET, Some(url)) => Request::get(&url),
            (RequestType::POST, Some(url)) => Request::post(&url),
            _ => return Err(Error::msg("method not set")),
        }.header("Content-Type", "application/json");

        let request = if let Some(data) = self.data {
            request.body(data)?
        } else {
            request.build()?
        };

        let resp = request.send().await?;
        let text = resp.text().await?;

        Ok(FetchResponse {
            payload: Some(serde_json::from_str::<T>(&text)?),
            status: Status::OK,
        })
    }

    pub fn fetch<Fut, T, F>(self, cb: F)
    where
        T: DeserializeOwned,
        F: FnOnce(FetchResponse<T>) -> Fut + 'static,
        Fut: Future<Output=()> + 'static,
    {
        spawn_local(async move {
            let resp = self.send_internal().await
                .unwrap_or_else(|e| FetchResponse::error(&e.to_string()));
            cb(resp).await;
        });
    }
}