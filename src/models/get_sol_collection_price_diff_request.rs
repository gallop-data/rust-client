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
pub struct GetSolCollectionPriceDiffRequest {
    /// The Gallop tag to identify the collection.
    #[serde(rename = "collection_tag")]
    pub collection_tag: String,
    /// The start date to pull data for calculations
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The end date to pull data for calculations
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Exclude suspected wash transactions?
    #[serde(rename = "exclude_wash", skip_serializing_if = "Option::is_none")]
    pub exclude_wash: Option<bool>,
}

impl GetSolCollectionPriceDiffRequest {
    pub fn new(collection_tag: String) -> GetSolCollectionPriceDiffRequest {
        GetSolCollectionPriceDiffRequest {
            collection_tag,
            start_date: None,
            end_date: None,
            exclude_wash: None,
        }
    }
}


