# GetSolDefaultCollectionRiskRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_tag** | **String** | The Gallop tag to identify the collection. | 
**holding_period** | **String** | The holding period to evaluate risk for, e.g. '12M' | 
**amount** | Option<**i32**> | The amount of tokens in your portfolio | [optional]
**rept_curr** | Option<**String**> | The currency to report results in | [optional]
**drawdown** | Option<**bool**> | If true, report drawdown volatility (based on negative returns only). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


