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
pub struct GetEthTokenTransactionsRequest {
    /// The contract address the token belongs to.
    #[serde(rename = "collection_address")]
    pub collection_address: String,
    /// The token id.
    #[serde(rename = "token_id")]
    pub token_id: String,
    /// The pagination cursor.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// The number of records returned per page.
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<PageSize>,
    /// The earliest block timestamp.
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The oldest block number to return.
    #[serde(rename = "start_block_number", skip_serializing_if = "Option::is_none")]
    pub start_block_number: Option<i32>,
    /// The latest block timestamp.
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl GetEthTokenTransactionsRequest {
    pub fn new(collection_address: String, token_id: String) -> GetEthTokenTransactionsRequest {
        GetEthTokenTransactionsRequest {
            collection_address,
            token_id,
            page: None,
            page_size: None,
            start_date: None,
            start_block_number: None,
            end_date: None,
        }
    }
}

/// The number of records returned per page.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PageSize {
    #[serde(rename = "50")]
    Variant50,
    #[serde(rename = "100")]
    Variant100,
    #[serde(rename = "500")]
    Variant500,
    #[serde(rename = "1000")]
    Variant1000,
}

impl Default for PageSize {
    fn default() -> PageSize {
        Self::Variant50
    }
}

