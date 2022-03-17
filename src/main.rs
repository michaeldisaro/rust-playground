use azure_data_cosmos::{
    clients::CosmosClient,
    prelude::{AuthorizationToken, CosmosOptions, Query},
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let result = get_organization_name("").await;
    println!("---> {:?}", result);
}

async fn get_organization_name(service_id: &str) -> Service {
    query(
        Config {
            account: String::from(""),
            key: String::from(""),
            db_name: String::from(""),
            collection: String::from(""),
        },
        service_id,
    )
        .await
}


async fn query(config: Config, service_id: &str) -> Service {
    let master_key = String::from(config.key);
    let database_name = String::from(config.db_name);
    let collection_name = String::from(config.collection);
    let account: String = String::from(config.account);

    let authorization_token = AuthorizationToken::primary_from_base64(&master_key).expect("NUOOOO");
    let options = CosmosOptions::default();
    let client = CosmosClient::new(account, authorization_token, options);
    let database_client = client.into_database_client(database_name);
    let collection_client = database_client.into_collection_client(collection_name);

    let p = collection_client
        .query_documents()
        .execute::<Service, Query>(Query::new(&format!(
            "SELECT TOP 1 * FROM c WHERE c.serviceId = '{}'",
            service_id
        )))
        .await
        .expect("ERROREEEEE");

    p.into_raw()
        .results
        .get(0)
        .ok_or("NOT FOUNDDDDDD")
        .cloned()
        .unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    serviceName: String,
    organizationName: String,
}

pub struct Config {
    account: String,
    key: String,
    db_name: String,
    collection: String,
}