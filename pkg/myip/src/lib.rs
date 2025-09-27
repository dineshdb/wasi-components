use spin_sdk::http::{Request, Response, send};

#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn get_ip() -> Result<String, String> {
        spin_executor::run(async move {
            let mut request = Request::get("https://1.1.1.1/cdn-cgi/trace");
            let response: Response = send(request.build()).await.map_err(|e| e.to_string())?;

            let status = response.status();
            if !(200..300).contains(status) {
                return Err(format!("Request failed with status code: {status}"));
            }

            let body = String::from_utf8_lossy(response.body());
            let text = body.into_owned();

            // Parse the response to extract IP address
            let ip = text
                .lines()
                .find(|line| line.starts_with("ip="))
                .map(|line| line.trim_start_matches("ip="))
                .ok_or_else(|| "Could not find IP address in response".to_string())?;

            Ok(ip.to_string())
        })
    }
}

bindings::export!(Component with_types_in bindings);
