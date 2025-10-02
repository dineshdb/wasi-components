use spin_sdk::http::{Method as SMethod, Request, Response, send};
use std::collections::HashMap;

pub async fn get(url: &str, headers: &HashMap<String, String>) -> anyhow::Result<Response> {
    let response: Response = fetch(url, Method::Get, headers)
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
    let mut request = Request::new(method.into(), url);
    for (name, value) in headers {
        request.set_header(name, value);
    }

    let response: Response = send(request)
        .await
        .map_err(|e| anyhow::anyhow!("error sending request: {e}"))?;
    let status = response.status();
    if !(200..300).contains(status) {
        return Err(anyhow::anyhow!("Request failed with status code: {status}"));
    }
    Ok(response)
}
