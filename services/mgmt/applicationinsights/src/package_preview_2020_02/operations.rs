#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
pub mod operations {
    use crate::models::*;
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<OperationsListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/providers/microsoft.insights/operations", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).map_err(|source| list::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| list::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| list::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| list::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: OperationsListResult = serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Ok(rsp_value)
            }
            status_code => Err(list::Error::DefaultResponse { status_code }),
        }
    }
    pub mod list {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse { status_code: http::StatusCode },
            #[error("Failed to parse request URL: {}", source)]
            ParseUrlError { source: url::ParseError },
            #[error("Failed to build request: {}", source)]
            BuildRequestError { source: http::Error },
            #[error("Failed to execute request: {}", source)]
            ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
            #[error("Failed to serialize request body: {}", source)]
            SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
            #[error("Failed to deserialize response body: {}", source)]
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            #[error("Failed to get access token: {}", source)]
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
pub async fn get_test_result_file(
    operation_config: &crate::OperationConfig,
    resource_group_name: &str,
    subscription_id: &str,
    web_test_name: &str,
    geo_location_id: &str,
    time_stamp: i64,
    download_as: &str,
    test_successful_criteria: Option<bool>,
    continuation_token: Option<&str>,
) -> std::result::Result<TestResultFileResponse, get_test_result_file::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!(
        "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.insights/webtests/{}/getTestResultFile",
        operation_config.base_path(),
        subscription_id,
        resource_group_name,
        web_test_name
    );
    let mut url = url::Url::parse(url_str).map_err(|source| get_test_result_file::Error::ParseUrlError { source })?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::POST);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .map_err(|source| get_test_result_file::Error::GetTokenError { source })?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
    url.query_pairs_mut().append_pair("geoLocationId", geo_location_id);
    url.query_pairs_mut().append_pair("timeStamp", time_stamp.to_string().as_str());
    url.query_pairs_mut().append_pair("downloadAs", download_as);
    if let Some(test_successful_criteria) = test_successful_criteria {
        url.query_pairs_mut()
            .append_pair("testSuccessfulCriteria", test_successful_criteria.to_string().as_str());
    }
    if let Some(continuation_token) = continuation_token {
        url.query_pairs_mut().append_pair("continuationToken", continuation_token);
    }
    let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
    req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder
        .body(req_body)
        .map_err(|source| get_test_result_file::Error::BuildRequestError { source })?;
    let rsp = http_client
        .execute_request(req)
        .await
        .map_err(|source| get_test_result_file::Error::ExecuteRequestError { source })?;
    match rsp.status() {
        http::StatusCode::OK => {
            let rsp_body = rsp.body();
            let rsp_value: TestResultFileResponse =
                serde_json::from_slice(rsp_body).map_err(|source| get_test_result_file::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
            Ok(rsp_value)
        }
        status_code => {
            let rsp_body = rsp.body();
            let rsp_value: ErrorResponse =
                serde_json::from_slice(rsp_body).map_err(|source| get_test_result_file::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
            Err(get_test_result_file::Error::DefaultResponse {
                status_code,
                value: rsp_value,
            })
        }
    }
}
pub mod get_test_result_file {
    use crate::{models, models::*};
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL: {}", source)]
        ParseUrlError { source: url::ParseError },
        #[error("Failed to build request: {}", source)]
        BuildRequestError { source: http::Error },
        #[error("Failed to execute request: {}", source)]
        ExecuteRequestError { source: Box<dyn std::error::Error + Sync + Send> },
        #[error("Failed to serialize request body: {}", source)]
        SerializeError { source: Box<dyn std::error::Error + Sync + Send> },
        #[error("Failed to deserialize response body: {}", source)]
        DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        #[error("Failed to get access token: {}", source)]
        GetTokenError { source: azure_core::errors::AzureError },
    }
}
