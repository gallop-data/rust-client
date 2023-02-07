# GetEthDefaultCollectionRiskRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_address** | **String** | The contract address of the token collection. | 
**holding_period** | **String** | The holding period to evaluate risk for, e.g. '12M' | 
**rept_curr** | Option<**String**> | The currency to report results in | [optional]
**amount** | Option<**i32**> | The amount of tokens in your portfolio | [optional]
**drawdown** | Option<**bool**> | If true, report drawdown volatility (based on negative returns only). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


