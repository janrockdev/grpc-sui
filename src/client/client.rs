use proto::model_client::ModelClient;
use std::error::Error;

pub mod proto {
    tonic::include_proto!("model");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:50051";
    let mut client = ModelClient::connect(url).await?;

    // Create request with collection and filter
    let req = proto::ModelAssetRequest { collection: "users".to_string(), filter: "rock".to_string()};

    let request = tonic::Request::new(req);
    let response = client.asset(request).await?;

    println!("Response: {:?} in {:?}ms.", response.get_ref().payload, &response.get_ref().duration);

    Ok(())
}
