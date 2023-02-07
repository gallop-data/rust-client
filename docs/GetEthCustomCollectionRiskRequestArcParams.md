# GetEthCustomCollectionRiskRequestArcParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mean** | Option<**String**> | Estimator for the location model of the time series, e.g: `Zero`, `Constant`, `ARX`, ... . | [optional]
**lags** | Option<**i32**> | Number of time time period lags considered. Note that the time period is set by the `frequency` parameter, so a value of 2 will assume 2-day lags if `frequency=='1D'`. | [optional]
**vol** | Option<**String**> | Estimator used for the volatility process of the time series, e.g: `Constant`, `ARCH`, `GARCH`, ...  | [optional]
**p** | Option<**i32**> | Order of the symmetric innovation(s). | [optional]
**dist** | Option<**String**> | Return distribution assumed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


