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
pub struct GetSolWashTradeRequest {
    /// The first signature to valildate.
    #[serde(rename = "first_signature")]
    pub first_signature: String,
}

impl GetSolWashTradeRequest {
    pub fn new(first_signature: String) -> GetSolWashTradeRequest {
        GetSolWashTradeRequest {
            first_signature,
        }
    }
}


