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
pub struct GetEthCollectionForecastsRequest {
    /// The contract address of the token collection.
    #[serde(rename = "collection_address")]
    pub collection_address: String,
    /// The collection percentile(s)
    #[serde(rename = "percentiles", skip_serializing_if = "Option::is_none")]
    pub percentiles: Option<Vec<i32>>,
    /// Type of statistical forecasting model to be calculated as a 3-char string, e.g. `arc` for ARCH
    #[serde(rename = "voltype", skip_serializing_if = "Option::is_none")]
    pub voltype: Option<Voltype>,
    /// The forecast horizon (i.e. the number of periods to forecast out)
    #[serde(rename = "horizon", skip_serializing_if = "Option::is_none")]
    pub horizon: Option<i32>,
    /// The interval at which to calculate returns to base the forecasts upon, e.g. `1D` for daily, `1M` for monthly etc.
    #[serde(rename = "frequency", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// The distribution assumed to calculate parametric risk for.
    #[serde(rename = "dist", skip_serializing_if = "Option::is_none")]
    pub dist: Option<Dist>,
    /// The start date to pull data for calculations
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The end date to pull data for calculations
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Set to true, returns confidencve intervals at alpha significance for price on top of forecasts for returns and volatilities
    #[serde(rename = "return_price_forecasts", skip_serializing_if = "Option::is_none")]
    pub return_price_forecasts: Option<bool>,
    /// The significance level, e.g. 0.05 for 95% confidence
    #[serde(rename = "alpha", skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f32>,
    /// The currency to report results in
    #[serde(rename = "rept_curr", skip_serializing_if = "Option::is_none")]
    pub rept_curr: Option<ReptCurr>,
    /// Exclude suspected wash transactions?
    #[serde(rename = "exclude_wash", skip_serializing_if = "Option::is_none")]
    pub exclude_wash: Option<bool>,
    #[serde(rename = "arch_params", skip_serializing_if = "Option::is_none")]
    pub arch_params: Option<Box<crate::models::GetEthCollectionForecastsRequestArchParams>>,
}

impl GetEthCollectionForecastsRequest {
    pub fn new(collection_address: String) -> GetEthCollectionForecastsRequest {
        GetEthCollectionForecastsRequest {
            collection_address,
            percentiles: None,
            voltype: None,
            horizon: None,
            frequency: None,
            dist: None,
            start_date: None,
            end_date: None,
            return_price_forecasts: None,
            alpha: None,
            rept_curr: None,
            exclude_wash: None,
            arch_params: None,
        }
    }
}

/// Type of statistical forecasting model to be calculated as a 3-char string, e.g. `arc` for ARCH
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Voltype {
    #[serde(rename = "arc")]
    Arc,
    #[serde(rename = "gar")]
    Gar,
    #[serde(rename = "har")]
    Har,
}

impl Default for Voltype {
    fn default() -> Voltype {
        Self::Arc
    }
}
/// The distribution assumed to calculate parametric risk for.
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
}

impl Default for ReptCurr {
    fn default() -> ReptCurr {
        Self::Eth
    }
}
