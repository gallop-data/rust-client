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
pub struct GetEthCollectionSalesOhlcRequest {
    /// The Ethereum contract address to identify the collection.
    #[serde(rename = "collection_address")]
    pub collection_address: String,
    /// The interval at which to return OHLC, e.g. `1D` for daily, `1M` for monthly etc.
    #[serde(rename = "frequency", skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// An attribute of the NFT to group results by.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
    /// If 'true', response dicts contain OHLCV
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<bool>,
    /// If 'true', append number of trades to OHLC(V)
    #[serde(rename = "n_trades", skip_serializing_if = "Option::is_none")]
    pub n_trades: Option<bool>,
    /// The currency to report results in
    #[serde(rename = "rept_curr", skip_serializing_if = "Option::is_none")]
    pub rept_curr: Option<ReptCurr>,
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

impl GetEthCollectionSalesOhlcRequest {
    pub fn new(collection_address: String) -> GetEthCollectionSalesOhlcRequest {
        GetEthCollectionSalesOhlcRequest {
            collection_address,
            frequency: None,
            group_by: None,
            volume: None,
            n_trades: None,
            rept_curr: None,
            start_date: None,
            end_date: None,
            exclude_wash: None,
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

