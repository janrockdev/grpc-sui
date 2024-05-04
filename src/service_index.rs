use bson::doc;
use mongodb::bson::{self};
use mongodb::{Client, Collection, Database};
use std::time::Instant;

use crate::proto::ModelAssetRequest;
use crate::model::users::users::User;
use self::proto::ModelAssetResponse;

mod proto {
    tonic::include_proto!("model");
}

async fn get_mongo_client() -> Database {
    let conn_var = std::env::var_os("MONGODB_URL")
        .expect("missing environment variable MONGODB_URL")
        .to_str()
        .expect("missing MONGODB_URL")
        .to_owned();
    let db_var = std::env::var_os("MONGODB_DATABASE")
        .expect("missing environment variable MONGODB_DATABASE")
        .to_str()
        .expect("missing MONGODB_DATABASE")
        .to_owned();
    let client = Client::with_uri_str(&conn_var).await.ok().unwrap();
    let database: Database = client.database(&db_var);

    database
}

pub(crate) async fn get_record(request: &ModelAssetRequest) -> Result<ModelAssetResponse, Box<dyn std::error::Error>> {
    let start = Instant::now();

    let database: Database = get_mongo_client().await;
    let collection: Collection<User> = database.collection(&request.collection.to_owned());

    let filter = doc! { "userName" : &request.filter.to_owned()};
    let result = collection.find_one(filter, None).await;
    let mut record: User = User {
        id: bson::oid::ObjectId::new(),
        user_name: "".to_string(),
        user_password: "".to_string(),
        user_email: "".to_string(),
        user_role: "".to_string(),
        created: chrono::Utc::now(),
        updated: chrono::Utc::now(),
        email_verified: false,
        active: false,
    };

    match result {
        Ok(Some(res)) => {
            record = res;
        }
        Ok(_) => println!("{:?}", result),
        //Ok(None) => println!("No document found"),
        Err(status) => eprint!("{}", status),
    }

    let response = proto::ModelAssetResponse {
        payload: serde_json::to_string(&record).unwrap(),
        duration: start.elapsed().as_millis() as u64,
    };

    Ok(response)
}
