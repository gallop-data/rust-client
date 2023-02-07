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
pub struct GetEthLeaderBoardRequest {
    /// The pagination cursor.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// The number of records returned per page.
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<PageSize>,
    /// The requested time interval
    #[serde(rename = "interval")]
    pub interval: Interval,
    /// The requested calculation metric
    #[serde(rename = "ranking_metric")]
    pub ranking_metric: RankingMetric,
}

impl GetEthLeaderBoardRequest {
    pub fn new(interval: Interval, ranking_metric: RankingMetric) -> GetEthLeaderBoardRequest {
        GetEthLeaderBoardRequest {
            page: None,
            page_size: None,
            interval,
            ranking_metric,
        }
    }
}

/// The number of records returned per page.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PageSize {
    #[serde(rename = "50")]
    Variant50,
    #[serde(rename = "100")]
    Variant100,
    #[serde(rename = "500")]
    Variant500,
    #[serde(rename = "1000")]
    Variant1000,
}

impl Default for PageSize {
    fn default() -> PageSize {
        Self::Variant50
    }
}
/// The requested time interval
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Interval {
    #[serde(rename = "one_day")]
    OneDay,
    #[serde(rename = "seven_days")]
    SevenDays,
    #[serde(rename = "thirty_days")]
    ThirtyDays,
    #[serde(rename = "ninety_days")]
    NinetyDays,
    #[serde(rename = "all_time")]
    AllTime,
}

impl Default for Interval {
    fn default() -> Interval {
        Self::OneDay
    }
}
/// The requested calculation metric
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RankingMetric {
    #[serde(rename = "eth_volume")]
    EthVolume,
    #[serde(rename = "sales_count")]
    SalesCount,
}

impl Default for RankingMetric {
    fn default() -> RankingMetric {
        Self::EthVolume
    }
}
