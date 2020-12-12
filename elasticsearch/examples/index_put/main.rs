use url::Url;
use elasticsearch::{
    auth::Credentials,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    Elasticsearch, Error, SearchParts, DEFAULT_ADDRESS,
    IndexParts
};
use serde_json::{json, Value};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("http://172.18.11.36:9200")?;
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
    let client = Elasticsearch::new(transport);

    let response = client
        .index(IndexParts::IndexId("test_db", "1"))
        .body(json!({
        "block_hash": "0x23",

    }))
        .send()
        .await?;

    let successful = response.status_code().is_success();
    Ok(())
}