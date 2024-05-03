use proto::model_client::ModelClient;
use std::error::Error;

pub mod proto {
    tonic::include_proto!("model");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50051";
    let mut client = ModelClient::connect(url).await?;
    let req = proto::ModelAssetRequest { request: "1".to_string() };
    let request = tonic::Request::new(req);
    let response = client.asset(request).await?;

    println!("Response: {:?} in {:?}", response.get_ref().payload, &response.get_ref().duration);

    Ok(())
}
