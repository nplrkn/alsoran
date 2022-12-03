#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ConnectionInfo {
    #[serde(rename = "operationType")]
    pub operation_type: models::OperationType,

    #[serde(rename = "ipAddress")]
    pub ip_address: String,

}

impl ConnectionInfo {
    #[allow(clippy::new_without_default)]
    pub fn new(operation_type: models::OperationType, ip_address: String, ) -> ConnectionInfo {
        ConnectionInfo {
            operation_type,
            ip_address,
        }
    }
}

/// Converts the ConnectionInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ConnectionInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping operationType in query parameter serialization


            Some("ipAddress".to_string()),
            Some(self.ip_address.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ConnectionInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ConnectionInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub operation_type: Vec<models::OperationType>,
            pub ip_address: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ConnectionInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "operationType" => intermediate_rep.operation_type.push(<models::OperationType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ipAddress" => intermediate_rep.ip_address.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ConnectionInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ConnectionInfo {
            operation_type: intermediate_rep.operation_type.into_iter().next().ok_or_else(|| "operationType missing in ConnectionInfo".to_string())?,
            ip_address: intermediate_rep.ip_address.into_iter().next().ok_or_else(|| "ipAddress missing in ConnectionInfo".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ConnectionInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ConnectionInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ConnectionInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ConnectionInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ConnectionInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ConnectionInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ConnectionInfo - {}",
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


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum OperationType {
    #[serde(rename = "setupNg")]
    SetupNg,
    #[serde(rename = "joinNg")]
    JoinNg,
    #[serde(rename = "addF1")]
    AddF1,
    #[serde(rename = "addE1")]
    AddE1,
}

impl std::fmt::Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OperationType::SetupNg => write!(f, "setupNg"),
            OperationType::JoinNg => write!(f, "joinNg"),
            OperationType::AddF1 => write!(f, "addF1"),
            OperationType::AddE1 => write!(f, "addE1"),
        }
    }
}

impl std::str::FromStr for OperationType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "setupNg" => std::result::Result::Ok(OperationType::SetupNg),
            "joinNg" => std::result::Result::Ok(OperationType::JoinNg),
            "addF1" => std::result::Result::Ok(OperationType::AddF1),
            "addE1" => std::result::Result::Ok(OperationType::AddE1),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Revision number of the worker refresh just sent by this worker
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RevisionNumber(i32);

impl std::convert::From<i32> for RevisionNumber {
    fn from(x: i32) -> Self {
        RevisionNumber(x)
    }
}

impl std::convert::From<RevisionNumber> for i32 {
    fn from(x: RevisionNumber) -> Self {
        x.0
    }
}

impl std::ops::Deref for RevisionNumber {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl std::ops::DerefMut for RevisionNumber {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}

