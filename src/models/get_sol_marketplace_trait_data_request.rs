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
pub struct GetSolMarketplaceTraitDataRequest {
    /// Collection_tag
    #[serde(rename = "collection_tag")]
    pub collection_tag: String,
}

impl GetSolMarketplaceTraitDataRequest {
    pub fn new(collection_tag: String) -> GetSolMarketplaceTraitDataRequest {
        GetSolMarketplaceTraitDataRequest {
            collection_tag,
        }
    }
}


