# GetSolCustomCollectionRiskRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_tag** | **String** | The Gallop tag to identify the collection. | 
**holding_period** | **String** | The holding period to evaluate risk for, e.g. `12M` | 
**percentiles** | Option<**Vec<i32>**> | The collection percentile(s) | [optional]
**amount** | Option<**i32**> | The amount of tokens in your portfolio | [optional]
**dist** | Option<**String**> | The distribution assumed to calculate parametric risk for | [optional]
**start_date** | Option<**String**> | The start date to pull data for calculations | [optional]
**end_date** | Option<**String**> | The end date to pull data for calculations | [optional]
**wins_outliers** | Option<**bool**> | Whether to winsorize time series outliers prior to calculating risk | [optional]
**risk_free_rate** | Option<**f32**> | The rate of return for an asset deemed risk free in the contemplated holding period | [optional]
**frequency** | Option<**String**> | The interval at which to calculate returns to base the forecasts upon, e.g. `1D` for daily, `1M` for monthly etc. | [optional]
**rept_curr** | Option<**String**> | The currency to report results in | [optional]
**exclude_wash** | Option<**bool**> | Exclude suspected wash transactions? | [optional]
**drawdown** | Option<**bool**> | If true, report drawdown volatility (based on negative returns only). | [optional]
**arc_params** | Option<[**crate::models::GetEthCustomCollectionRiskRequestArcParams**](getEthCustomCollectionRisk_request_arc_params.md)> |  | [optional]
**gar_params** | Option<[**crate::models::GetEthCustomCollectionRiskRequestGarParams**](getEthCustomCollectionRisk_request_gar_params.md)> |  | [optional]
**har_params** | Option<[**crate::models::GetEthCustomCollectionRiskRequestHarParams**](getEthCustomCollectionRisk_request_har_params.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


