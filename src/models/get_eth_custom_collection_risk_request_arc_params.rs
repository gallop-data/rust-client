/*
 * Gallop API
 *
 * Data and insights APIs, webooks, and dashboards enabling businesses to launch tokenized products in seconds.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@higallop.com
 * Generated by: https://openapi-generator.tech
 */

/// GetEthCustomCollectionRiskRequestArcParams : JSON containing options for the ARCH model.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetEthCustomCollectionRiskRequestArcParams {
    /// Estimator for the location model of the time series, e.g: `Zero`, `Constant`, `ARX`, ... .
    #[serde(rename = "mean", skip_serializing_if = "Option::is_none")]
    pub mean: Option<String>,
    /// Number of time time period lags considered. Note that the time period is set by the `frequency` parameter, so a value of 2 will assume 2-day lags if `frequency=='1D'`.
    #[serde(rename = "lags", skip_serializing_if = "Option::is_none")]
    pub lags: Option<i32>,
    /// Estimator used for the volatility process of the time series, e.g: `Constant`, `ARCH`, `GARCH`, ... 
    #[serde(rename = "vol", skip_serializing_if = "Option::is_none")]
    pub vol: Option<String>,
    /// Order of the symmetric innovation(s).
    #[serde(rename = "p", skip_serializing_if = "Option::is_none")]
    pub p: Option<i32>,
    /// Return distribution assumed.
    #[serde(rename = "dist", skip_serializing_if = "Option::is_none")]
    pub dist: Option<Dist>,
}

impl GetEthCustomCollectionRiskRequestArcParams {
    /// JSON containing options for the ARCH model.
    pub fn new() -> GetEthCustomCollectionRiskRequestArcParams {
        GetEthCustomCollectionRiskRequestArcParams {
            mean: None,
            lags: None,
            vol: None,
            p: None,
            dist: None,
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
