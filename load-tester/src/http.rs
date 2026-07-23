use crate::payload::PayloadSpec;
use http::StatusCode;
/// Sends out requests
use reqwest::{Client, Url};

pub async fn request(
    client: &Client,
    url: &Url,
    method: &reqwest::Method,
    payload: &Option<PayloadSpec>,
) -> Result<(StatusCode, String), anyhow::Error> {
    let mut request = client.request(method.clone(), url.clone());

    if let Some(body) = payload {
        match body {
            // PayloadSpec::Bytes(items) => {
            //     request = request.body(items.clone());
            // }
            PayloadSpec::Json(value) => {
                request = request.json(value);
            } // PayloadSpec::Template { fields } => todo!(),
        }
    }

    let response = request.send().await?;

    let status = response.status();
    let text = response.text().await?;
    println!("{}", text);

    Ok((status, text))
}
