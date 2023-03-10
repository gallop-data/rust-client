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
pub struct GetPolCustomTokenRiskRequest {
    /// The contract address of the token collection.
    #[serde(rename = "collection_address")]
    pub collection_address: String,
    /// The id(s) for the token(s).
    #[serde(rename = "token_id")]
    pub token_id: Vec<String>,
    /// The holding period to evaluate risk for, e.g. `12M`
    #[serde(rename = "holding_period")]
    pub holding_period: String,
    /// Return distribution assumed.
    #[serde(rename = "dist", skip_serializing_if = "Option::is_none")]
    pub dist: Option<Dist>,
    /// The start date to pull data for calculations
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The end date to pull data for calculations
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// The rate of return for an asset deemed risk free in the contemplated holding period
    #[serde(rename = "risk_free_rate", skip_serializing_if = "Option::is_none")]
    pub risk_free_rate: Option<f32>,
    /// Whether to winsorize time series outliers prior to calculating risk
    #[serde(rename = "wins_outliers", skip_serializing_if = "Option::is_none")]
    pub wins_outliers: Option<bool>,
    /// The interval at which to calculate returns to base the forecasts upon, e.g. `1D` for daily, `1M` for monthly etc.
    #[serde(rename = "frequency", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// The currency to report results in
    #[serde(rename = "rept_curr", skip_serializing_if = "Option::is_none")]
    pub rept_curr: Option<ReptCurr>,
    /// Exclude suspected wash transactions?
    #[serde(rename = "exclude_wash", skip_serializing_if = "Option::is_none")]
    pub exclude_wash: Option<bool>,
    /// If true, report drawdown volatility (based on negative returns only).
    #[serde(rename = "drawdown", skip_serializing_if = "Option::is_none")]
    pub drawdown: Option<bool>,
    #[serde(rename = "arc_params", skip_serializing_if = "Option::is_none")]
    pub arc_params: Option<Box<crate::models::GetEthCustomCollectionRiskRequestArcParams>>,
    #[serde(rename = "gar_params", skip_serializing_if = "Option::is_none")]
    pub gar_params: Option<Box<crate::models::GetEthCustomCollectionRiskRequestGarParams>>,
    #[serde(rename = "har_params", skip_serializing_if = "Option::is_none")]
    pub har_params: Option<Box<crate::models::GetEthCustomCollectionRiskRequestHarParams>>,
}

impl GetPolCustomTokenRiskRequest {
    pub fn new(collection_address: String, token_id: Vec<String>, holding_period: String) -> GetPolCustomTokenRiskRequest {
        GetPolCustomTokenRiskRequest {
            collection_address,
            token_id,
            holding_period,
            dist: None,
            start_date: None,
            end_date: None,
            risk_free_rate: None,
            wins_outliers: None,
            frequency: None,
            rept_curr: None,
            exclude_wash: None,
            drawdown: None,
            arc_params: None,
            gar_params: None,
            har_params: None,
        }
    }
}

/// Return distribution assumed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Dist {
    #[serde(rename = "norm")]
    Norm,
    #[serde(rename = "t")]
    T,
}

impl Default for Dist {
    fn default() -> Dist {
        Self::Norm
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

