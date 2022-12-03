#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ConnectionState {
    #[serde(rename = "ngUp")]
    pub ng_up: bool,

    #[serde(rename = "e1Up")]
    pub e1_up: bool,

    #[serde(rename = "f1Up")]
    pub f1_up: bool,

}

impl ConnectionState {
    #[allow(clippy::new_without_default)]
    pub fn new(ng_up: bool, e1_up: bool, f1_up: bool, ) -> ConnectionState {
        ConnectionState {
            ng_up,
            e1_up,
            f1_up,
        }
    }
}

/// Converts the ConnectionState value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ConnectionState {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("ngUp".to_string()),
            Some(self.ng_up.to_string()),


            Some("e1Up".to_string()),
            Some(self.e1_up.to_string()),


            Some("f1Up".to_string()),
            Some(self.f1_up.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ConnectionState value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ConnectionState {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub ng_up: Vec<bool>,
            pub e1_up: Vec<bool>,
            pub f1_up: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ConnectionState".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "ngUp" => intermediate_rep.ng_up.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "e1Up" => intermediate_rep.e1_up.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "f1Up" => intermediate_rep.f1_up.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ConnectionState".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ConnectionState {
            ng_up: intermediate_rep.ng_up.into_iter().next().ok_or_else(|| "ngUp missing in ConnectionState".to_string())?,
            e1_up: intermediate_rep.e1_up.into_iter().next().ok_or_else(|| "e1Up missing in ConnectionState".to_string())?,
            f1_up: intermediate_rep.f1_up.into_iter().next().ok_or_else(|| "f1Up missing in ConnectionState".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ConnectionState> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ConnectionState>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ConnectionState>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ConnectionState - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ConnectionState> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ConnectionState as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ConnectionState - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Error(String);

impl std::convert::From<String> for Error {
    fn from(x: String) -> Self {
        Error(x)
    }
}

impl std::string::ToString for Error {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for Error {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Error(x.to_string()))
    }
}

impl std::convert::From<Error> for String {
    fn from(x: Error) -> Self {
        x.0
    }
}

impl std::ops::Deref for Error {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Error {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct IpAddress(String);

impl std::convert::From<String> for IpAddress {
    fn from(x: String) -> Self {
        IpAddress(x)
    }
}

impl std::string::ToString for IpAddress {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for IpAddress {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(IpAddress(x.to_string()))
    }
}

impl std::convert::From<IpAddress> for String {
    fn from(x: IpAddress) -> Self {
        x.0
    }
}

impl std::ops::Deref for IpAddress {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for IpAddress {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RefreshWorker {
    #[serde(rename = "workerId")]
    pub worker_id: uuid::Uuid,

    #[serde(rename = "revisionNumber")]
    pub revision_number: i32,

    #[serde(rename = "workerInfo")]
    pub worker_info: models::WorkerInfo,

    #[serde(rename = "connectionState")]
    pub connection_state: models::ConnectionState,

}

impl RefreshWorker {
    #[allow(clippy::new_without_default)]
    pub fn new(worker_id: uuid::Uuid, revision_number: i32, worker_info: models::WorkerInfo, connection_state: models::ConnectionState, ) -> RefreshWorker {
        RefreshWorker {
            worker_id,
            revision_number,
            worker_info,
            connection_state,
        }
    }
}

/// Converts the RefreshWorker value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RefreshWorker {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping workerId in query parameter serialization


            Some("revisionNumber".to_string()),
            Some(self.revision_number.to_string()),

            // Skipping workerInfo in query parameter serialization

            // Skipping connectionState in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RefreshWorker value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RefreshWorker {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub worker_id: Vec<uuid::Uuid>,
            pub revision_number: Vec<i32>,
            pub worker_info: Vec<models::WorkerInfo>,
            pub connection_state: Vec<models::ConnectionState>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RefreshWorker".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "workerId" => intermediate_rep.worker_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "revisionNumber" => intermediate_rep.revision_number.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "workerInfo" => intermediate_rep.worker_info.push(<models::WorkerInfo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "connectionState" => intermediate_rep.connection_state.push(<models::ConnectionState as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RefreshWorker".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RefreshWorker {
            worker_id: intermediate_rep.worker_id.into_iter().next().ok_or_else(|| "workerId missing in RefreshWorker".to_string())?,
            revision_number: intermediate_rep.revision_number.into_iter().next().ok_or_else(|| "revisionNumber missing in RefreshWorker".to_string())?,
            worker_info: intermediate_rep.worker_info.into_iter().next().ok_or_else(|| "workerInfo missing in RefreshWorker".to_string())?,
            connection_state: intermediate_rep.connection_state.into_iter().next().ok_or_else(|| "connectionState missing in RefreshWorker".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RefreshWorker> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RefreshWorker>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RefreshWorker>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RefreshWorker - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RefreshWorker> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RefreshWorker as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RefreshWorker - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WorkerInfo {
    #[serde(rename = "connectionApiUrl")]
    pub connection_api_url: String,

    #[serde(rename = "f1Address")]
    pub f1_address: String,

    #[serde(rename = "e1Address")]
    pub e1_address: String,

}

impl WorkerInfo {
    #[allow(clippy::new_without_default)]
    pub fn new(connection_api_url: String, f1_address: String, e1_address: String, ) -> WorkerInfo {
        WorkerInfo {
            connection_api_url,
            f1_address,
            e1_address,
        }
    }
}

/// Converts the WorkerInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WorkerInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("connectionApiUrl".to_string()),
            Some(self.connection_api_url.to_string()),


            Some("f1Address".to_string()),
            Some(self.f1_address.to_string()),


            Some("e1Address".to_string()),
            Some(self.e1_address.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WorkerInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WorkerInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub connection_api_url: Vec<String>,
            pub f1_address: Vec<String>,
            pub e1_address: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing WorkerInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "connectionApiUrl" => intermediate_rep.connection_api_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "f1Address" => intermediate_rep.f1_address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "e1Address" => intermediate_rep.e1_address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing WorkerInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WorkerInfo {
            connection_api_url: intermediate_rep.connection_api_url.into_iter().next().ok_or_else(|| "connectionApiUrl missing in WorkerInfo".to_string())?,
            f1_address: intermediate_rep.f1_address.into_iter().next().ok_or_else(|| "f1Address missing in WorkerInfo".to_string())?,
            e1_address: intermediate_rep.e1_address.into_iter().next().ok_or_else(|| "e1Address missing in WorkerInfo".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WorkerInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WorkerInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<WorkerInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for WorkerInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WorkerInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <WorkerInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into WorkerInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

