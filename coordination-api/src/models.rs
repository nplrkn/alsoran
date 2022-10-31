#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

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


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TransportAddress {
    #[serde(rename = "host")]
    pub host: String,

    #[serde(rename = "port")]
    pub port: u16,

}

impl TransportAddress {
    #[allow(clippy::new_without_default)]
    pub fn new(host: String, port: u16, ) -> TransportAddress {
        TransportAddress {
            host,
            port,
        }
    }
}

/// Converts the TransportAddress value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TransportAddress {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("host".to_string()),
            Some(self.host.to_string()),


            Some("port".to_string()),
            Some(self.port.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TransportAddress value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TransportAddress {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub host: Vec<String>,
            pub port: Vec<u16>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TransportAddress".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "host" => intermediate_rep.host.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "port" => intermediate_rep.port.push(<u16 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TransportAddress".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TransportAddress {
            host: intermediate_rep.host.into_iter().next().ok_or_else(|| "host missing in TransportAddress".to_string())?,
            port: intermediate_rep.port.into_iter().next().ok_or_else(|| "port missing in TransportAddress".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TransportAddress> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TransportAddress>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TransportAddress>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TransportAddress - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TransportAddress> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TransportAddress as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TransportAddress - {}",
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
    #[serde(rename = "workerUniqueId")]
    pub worker_unique_id: uuid::Uuid,

    #[serde(rename = "connectionApiUrl")]
    pub connection_api_url: String,

    #[serde(rename = "f1Address")]
    pub f1_address: models::TransportAddress,

    #[serde(rename = "e1Address")]
    pub e1_address: models::TransportAddress,

    #[serde(rename = "connectedAmfs")]
    pub connected_amfs: Vec<String>,

    #[serde(rename = "connectedDus")]
    pub connected_dus: Vec<String>,

    #[serde(rename = "connectedUps")]
    pub connected_ups: Vec<String>,

}

impl WorkerInfo {
    #[allow(clippy::new_without_default)]
    pub fn new(worker_unique_id: uuid::Uuid, connection_api_url: String, f1_address: models::TransportAddress, e1_address: models::TransportAddress, connected_amfs: Vec<String>, connected_dus: Vec<String>, connected_ups: Vec<String>, ) -> WorkerInfo {
        WorkerInfo {
            worker_unique_id,
            connection_api_url,
            f1_address,
            e1_address,
            connected_amfs,
            connected_dus,
            connected_ups,
        }
    }
}

/// Converts the WorkerInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WorkerInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping workerUniqueId in query parameter serialization


            Some("connectionApiUrl".to_string()),
            Some(self.connection_api_url.to_string()),

            // Skipping f1Address in query parameter serialization

            // Skipping e1Address in query parameter serialization


            Some("connectedAmfs".to_string()),
            Some(self.connected_amfs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),


            Some("connectedDus".to_string()),
            Some(self.connected_dus.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),


            Some("connectedUps".to_string()),
            Some(self.connected_ups.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

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
            pub worker_unique_id: Vec<uuid::Uuid>,
            pub connection_api_url: Vec<String>,
            pub f1_address: Vec<models::TransportAddress>,
            pub e1_address: Vec<models::TransportAddress>,
            pub connected_amfs: Vec<Vec<String>>,
            pub connected_dus: Vec<Vec<String>>,
            pub connected_ups: Vec<Vec<String>>,
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
                    "workerUniqueId" => intermediate_rep.worker_unique_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "connectionApiUrl" => intermediate_rep.connection_api_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "f1Address" => intermediate_rep.f1_address.push(<models::TransportAddress as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "e1Address" => intermediate_rep.e1_address.push(<models::TransportAddress as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "connectedAmfs" => return std::result::Result::Err("Parsing a container in this style is not supported in WorkerInfo".to_string()),
                    "connectedDus" => return std::result::Result::Err("Parsing a container in this style is not supported in WorkerInfo".to_string()),
                    "connectedUps" => return std::result::Result::Err("Parsing a container in this style is not supported in WorkerInfo".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing WorkerInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WorkerInfo {
            worker_unique_id: intermediate_rep.worker_unique_id.into_iter().next().ok_or_else(|| "workerUniqueId missing in WorkerInfo".to_string())?,
            connection_api_url: intermediate_rep.connection_api_url.into_iter().next().ok_or_else(|| "connectionApiUrl missing in WorkerInfo".to_string())?,
            f1_address: intermediate_rep.f1_address.into_iter().next().ok_or_else(|| "f1Address missing in WorkerInfo".to_string())?,
            e1_address: intermediate_rep.e1_address.into_iter().next().ok_or_else(|| "e1Address missing in WorkerInfo".to_string())?,
            connected_amfs: intermediate_rep.connected_amfs.into_iter().next().ok_or_else(|| "connectedAmfs missing in WorkerInfo".to_string())?,
            connected_dus: intermediate_rep.connected_dus.into_iter().next().ok_or_else(|| "connectedDus missing in WorkerInfo".to_string())?,
            connected_ups: intermediate_rep.connected_ups.into_iter().next().ok_or_else(|| "connectedUps missing in WorkerInfo".to_string())?,
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

