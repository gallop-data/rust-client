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
pub struct GetPolTokensRequest {
    /// The contract address of the token collection.
    #[serde(rename = "collection_address")]
    pub collection_address: String,
    /// A list of token ids.
    #[serde(rename = "token_id", skip_serializing_if = "Option::is_none")]
    pub token_id: Option<Vec<String>>,
    /// The pagination cursor.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// The number of records returned per page.
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<PageSize>,
}

impl GetPolTokensRequest {
    pub fn new(collection_address: String) -> GetPolTokensRequest {
        GetPolTokensRequest {
            collection_address,
            token_id: None,
            page: None,
            page_size: None,
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

