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
pub struct GetEthHistoricalTransactionsRequest {
    /// The contract address of the collection.
    #[serde(rename = "collection_address")]
    pub collection_address: String,
    /// The id for the token.
    #[serde(rename = "token_id", skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    /// The pagination cursor.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
}

impl GetEthHistoricalTransactionsRequest {
    pub fn new(collection_address: String) -> GetEthHistoricalTransactionsRequest {
        GetEthHistoricalTransactionsRequest {
            collection_address,
            token_id: None,
            page: None,
        }
    }
}


