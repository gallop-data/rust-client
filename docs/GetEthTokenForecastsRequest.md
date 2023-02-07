# GetEthTokenForecastsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_address** | **String** | The contract address of the token collection. | 
**token_id** | **Vec<String>** | The id(s) for the token(s). | 
**voltype** | Option<**String**> | Type of statistical forecasting model to be calculated as a 3-char string, e.g. `arc` for ARCH | [optional]
**horizon** | Option<**i32**> | The forecast horizon (i.e. the number of periods to forecast out) | [optional]
**frequency** | Option<**String**> | The interval at which to calculate returns to base the forecasts upon, e.g. `1D` for daily, `1M` for monthly etc. | [optional]
**dist** | Option<**String**> | The distribution assumed to calculate parametric risk for. | [optional]
**start_date** | Option<**String**> | The start date to pull data for calculations | [optional]
**end_date** | Option<**String**> | The end date to pull data for calculations | [optional]
**return_price_forecasts** | Option<**bool**> | Set to True, returns confidencve intervals at alpha significance for price on top of forecasts for returns and volatilities | [optional]
**alpha** | Option<**f32**> | The significance level, e.g. 0.05 for 95% confidence | [optional]
**rept_curr** | Option<**String**> | The currency to report results in | [optional]
**exclude_wash** | Option<**bool**> | Exclude suspected wash transactions? | [optional]
**arch_params** | Option<[**crate::models::GetEthCollectionForecastsRequestArchParams**](getEthCollectionForecasts_request_arch_params.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


