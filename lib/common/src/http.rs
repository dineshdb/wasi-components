use spin_sdk::http::{Method as SMethod, Request, Response, send};
use std::collections::HashMap;

pub async fn get(url: &str, headers: &HashMap<String, String>) -> anyhow::Result<Response> {
    let mut request = Request::get(url);
    for (name, value) in headers {
        request.header(name, value);
    }

    let response: Response = send(request.build())
        .await
        .map_err(|e| anyhow::anyhow!("error sending request: {e}"))?;
    let status = response.status();
    if !(200..300).contains(status) {
        return Err(anyhow::anyhow!("Request failed with status code: {status}"));
    }
    Ok(response)
}

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl From<Method> for SMethod {
    fn from(value: Method) -> Self {
        match value {
            Method::Get => SMethod::Get,
            Method::Post => SMethod::Post,
            Method::Put => SMethod::Put,
            Method::Delete => SMethod::Delete,
        }
    }
}

pub async fn fetch(
    url: &str,
    method: Method,
    headers: &HashMap<String, String>,
) -> anyhow::Result<Response> {
    match method {
        Method::Get => get(url, headers).await,
        _ => Err(anyhow::anyhow!("Unsupported method for fetch without body")),
    }
}

pub async fn post_json(
    url: &str,
    headers: &HashMap<String, String>,
    body: Vec<u8>,
) -> anyhow::Result<Response> {
    let mut request = Request::post(url, body);
    for (name, value) in headers {
        request.header(name, value);
    }

    let response: Response = send(request.build())
        .await
        .map_err(|e| anyhow::anyhow!("error sending request: {e}"))?;
    let status = response.status();
    if !(200..300).contains(status) {
        return Err(anyhow::anyhow!("Request failed with status code: {status}"));
    }
    Ok(response)
}
