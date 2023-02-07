# GetSolTokenAppraisalRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mint_address** | **Vec<String>** | List of mint addresses of tokens to appraise | 
**rept_curr** | Option<**String**> | The currency to report results in | [optional]
**frequency** | Option<**String**> | The interval at which to calculate intermediate results and forecasts. | [optional]
**horizon** | Option<**i32**> | The forecast horizon (i.e. the number of periods to forecast out). Defaults to zero which only returns nowcasts. | [optional]
**alpha** | Option<**f32**> | The significance level for the liquidation estimate, e.g. 0.05 for 95% confidence | [optional]
**exclude_wash** | Option<**bool**> | Exclude suspected wash transactions? | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


