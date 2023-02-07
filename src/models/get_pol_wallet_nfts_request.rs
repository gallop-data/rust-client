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
pub struct GetPolWalletNftsRequest {
    /// The wallet address to search.
    #[serde(rename = "wallet_address")]
    pub wallet_address: String,
}

impl GetPolWalletNftsRequest {
    pub fn new(wallet_address: String) -> GetPolWalletNftsRequest {
        GetPolWalletNftsRequest {
            wallet_address,
        }
    }
}


