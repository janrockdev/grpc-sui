pub mod users {
    use serde::{Deserialize, Serialize};
    use bson::oid::ObjectId;
    use chrono::prelude::*;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct User {
        #[serde(rename = "_id")]
        pub id: ObjectId,
        #[serde(rename = "userEmail")]
        pub user_email: String,
        #[serde(rename = "userName")]
        pub user_name: String,
        #[serde(rename = "userPassword")]
        pub user_password: String,
        #[serde(rename = "userRole")]
        pub user_role: String,
        #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
        pub created: DateTime<Utc>,
        #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
        pub updated: DateTime<Utc>,
        #[serde(rename = "emailVerified")]
        pub email_verified: bool,
        pub active: bool,
    }
}