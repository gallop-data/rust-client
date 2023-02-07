# GetEthDefaultTokenRiskRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_address** | **String** | The contract address of the token collection. | 
**token_id** | **Vec<String>** | The id(s) for the token(s). | 
**holding_period** | **String** | The holding period to evaluate risk for, e.g. '12M' | 
**rept_curr** | Option<**String**> | The currency to report results in | [optional]
**drawdown** | Option<**bool**> | If true, report drawdown volatility (based on negative returns only). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


