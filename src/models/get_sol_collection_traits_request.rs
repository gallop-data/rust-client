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
pub struct GetSolCollectionTraitsRequest {
    /// The tag of the collection.
    #[serde(rename = "collection_tag", skip_serializing_if = "Option::is_none")]
    pub collection_tag: Option<String>,
}

impl GetSolCollectionTraitsRequest {
    pub fn new() -> GetSolCollectionTraitsRequest {
        GetSolCollectionTraitsRequest {
            collection_tag: None,
        }
    }
}

