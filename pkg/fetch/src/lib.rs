// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use spin_sdk::http::{Request, Response, send};

#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn fetch(url: String, headers: Vec<bindings::Header>) -> Result<String, String> {
        spin_executor::run(async move {
            let mut request = Request::get(url);
            for header in headers {
                request.header(header.name.clone().as_str(), header.value.as_str());
            }

            let response: Response = send(request.build()).await.map_err(|e| e.to_string())?;
            let status = response.status();
            if !(200..300).contains(status) {
                return Err(format!("Request failed with status code: {status}"));
            }
            let body = String::from_utf8_lossy(response.body());
            Ok(body.into_owned())
        })
    }
}

bindings::export!(Component with_types_in bindings);
