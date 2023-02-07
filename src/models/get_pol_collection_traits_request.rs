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
pub struct GetPolCollectionTraitsRequest {
    /// The contract address of the collection.
    #[serde(rename = "collection_address", skip_serializing_if = "Option::is_none")]
    pub collection_address: Option<String>,
}

impl GetPolCollectionTraitsRequest {
    pub fn new() -> GetPolCollectionTraitsRequest {
        GetPolCollectionTraitsRequest {
            collection_address: None,
        }
    }
}

