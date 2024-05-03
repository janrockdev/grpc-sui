use proto::admin_client::AdminClient;
use tonic::Request;
use std::{error::Error, str::FromStr};

pub mod proto {
    tonic::include_proto!("model");
}

fn get_token() -> String {
    String::from("Bearer some-secret-token")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let client = tonic::transport::Channel::from_static("http://[::1]:50051").connect().await?;

    let token = get_token();

    let mut client = AdminClient::with_interceptor(client, move |mut req: Request<()>| {
        req.metadata_mut().insert(
            "authorization",
            tonic::metadata::MetadataValue::from_str(&token).unwrap(),
        );
        Ok(req)
    });

    // Create request
    let req = proto::GetResultRequest {};

    // Create request with metadata
    let request = Request::new(req);

    // Send request
    let response = client.get_request_result(request).await?;

    println!("Response: {:?}", response.get_ref().count);


    Ok(())
}
