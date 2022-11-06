# default_api

All URIs are relative to *http://localhost/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
**addE1ap**](default_api.md#addE1ap) | **POST** /addE1ap | Instructs a worker to add another worker to an existing E1AP interface instance
**addF1ap**](default_api.md#addF1ap) | **POST** /addF1ap | Instructs a worker to add another worker to an existing F1AP interface instance
**joinNgap**](default_api.md#joinNgap) | **POST** /joinNgap | Instructs a worker to join an existing NGAP interface instance set up by another worker.
**setupNgap**](default_api.md#setupNgap) | **POST** /setupNgap | Instructs a worker to set up an NGAP interface instance with the AMF


# **addE1ap**
> addE1ap(body)
Instructs a worker to add another worker to an existing E1AP interface instance

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**string**](string.md)| Worker E1AP endpoint | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addF1ap**
> addF1ap(body)
Instructs a worker to add another worker to an existing F1AP interface instance

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**string**](string.md)| Worker F1AP endpoint | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **joinNgap**
> joinNgap(body)
Instructs a worker to join an existing NGAP interface instance set up by another worker.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**string**](string.md)| AMF endpoint | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **setupNgap**
> models::AmfInfo setupNgap(body)
Instructs a worker to set up an NGAP interface instance with the AMF

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**string**](string.md)| AMF endpoint | 

### Return type

[**models::AmfInfo**](AmfInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

