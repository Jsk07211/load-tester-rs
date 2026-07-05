use reqwest::{Client, Url};

pub async fn get_request(client: &Client, url: Url) -> Result<(), anyhow::Error> {
    let response = client.get(url).send().await?;
    print!("{}", response.text().await.unwrap());
    Ok(())
}
