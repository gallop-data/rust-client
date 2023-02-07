# GetSolDefaultTokenRiskRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mint_address** | **Vec<String>** | A token mint address or list of mint addresses. | 
**holding_period** | **String** | The holding period to evaluate risk for, e.g. '12M' | 
**rept_curr** | Option<**String**> | The currency to report results in | [optional]
**drawdown** | Option<**bool**> | If true, report drawdown volatility (based on negative returns only). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


