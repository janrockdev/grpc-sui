use reqwest::Client;

pub mod proto {
    tonic::include_proto!("model");
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "http://[::1]:50051";
    let response = Client::new().get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Rust")
        .bearer_auth("some-secret-token")
        .send()
        .await?;

    println!("{}", response.text().await?);
    Ok(())
}