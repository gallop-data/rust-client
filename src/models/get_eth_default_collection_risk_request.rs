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
pub struct GetEthDefaultCollectionRiskRequest {
    /// The contract address of the token collection.
    #[serde(rename = "collection_address")]
    pub collection_address: String,
    /// The holding period to evaluate risk for, e.g. '12M'
    #[serde(rename = "holding_period")]
    pub holding_period: String,
    /// The currency to report results in
    #[serde(rename = "rept_curr", skip_serializing_if = "Option::is_none")]
    pub rept_curr: Option<ReptCurr>,
    /// The amount of tokens in your portfolio
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// If true, report drawdown volatility (based on negative returns only).
    #[serde(rename = "drawdown", skip_serializing_if = "Option::is_none")]
    pub drawdown: Option<bool>,
}

impl GetEthDefaultCollectionRiskRequest {
    pub fn new(collection_address: String, holding_period: String) -> GetEthDefaultCollectionRiskRequest {
        GetEthDefaultCollectionRiskRequest {
            collection_address,
            holding_period,
            rept_curr: None,
            amount: None,
            drawdown: None,
        }
    }
}

/// The currency to report results in
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReptCurr {
    #[serde(rename = "eth")]
    Eth,
    #[serde(rename = "usd")]
    Usd,
}

impl Default for ReptCurr {
    fn default() -> ReptCurr {
        Self::Eth
    }
}

