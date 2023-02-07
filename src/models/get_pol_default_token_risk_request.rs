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
pub struct GetPolDefaultTokenRiskRequest {
    /// The contract address of the token collection.
    #[serde(rename = "collection_address")]
    pub collection_address: String,
    /// The id(s) for the token(s).
    #[serde(rename = "token_id")]
    pub token_id: Vec<String>,
    /// The holding period to evaluate risk for, e.g. '12M'
    #[serde(rename = "holding_period")]
    pub holding_period: String,
    /// The currency to report results in
    #[serde(rename = "rept_curr", skip_serializing_if = "Option::is_none")]
    pub rept_curr: Option<ReptCurr>,
    /// If true, report drawdown volatility (based on negative returns only).
    #[serde(rename = "drawdown", skip_serializing_if = "Option::is_none")]
    pub drawdown: Option<bool>,
}

impl GetPolDefaultTokenRiskRequest {
    pub fn new(collection_address: String, token_id: Vec<String>, holding_period: String) -> GetPolDefaultTokenRiskRequest {
        GetPolDefaultTokenRiskRequest {
            collection_address,
            token_id,
            holding_period,
            rept_curr: None,
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
    #[serde(rename = "matic")]
    Matic,
}

impl Default for ReptCurr {
    fn default() -> ReptCurr {
        Self::Eth
    }
}
