/*
 * Gallop API
 *
 * Data and insights APIs, webooks, and dashboards enabling businesses to launch tokenized products in seconds.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@higallop.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetPolWashTradeRequest {
    /// The transaction hash to valildate.
    #[serde(rename = "transaction_hash")]
    pub transaction_hash: String,
}

impl GetPolWashTradeRequest {
    pub fn new(transaction_hash: String) -> GetPolWashTradeRequest {
        GetPolWashTradeRequest {
            transaction_hash,
        }
    }
}

