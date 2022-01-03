#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Error {
    #[serde(rename = "code")]
    pub code: i32,

    #[serde(rename = "message")]
    pub message: String,

}

impl Error {
    pub fn new(code: i32, message: String, ) -> Error {
        Error {
            code: code,
            message: message,
        }
    }
}

/// Converts the Error value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("code".to_string());
        params.push(self.code.to_string());


        params.push("message".to_string());
        params.push(self.message.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Error value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Error {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub code: Vec<i32>,
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Error".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Error".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Error {
            code: intermediate_rep.code.into_iter().next().ok_or("code missing in Error".to_string())?,
            message: intermediate_rep.message.into_iter().next().ok_or("message missing in Error".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Error> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Error>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Error>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Error - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Error> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Error as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Error - {}",
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
pub struct InterfaceManagementReq {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "procedure")]
    pub procedure: String,

}

impl InterfaceManagementReq {
    pub fn new(procedure: String, ) -> InterfaceManagementReq {
        InterfaceManagementReq {
            procedure: procedure,
        }
    }
}

/// Converts the InterfaceManagementReq value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for InterfaceManagementReq {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("procedure".to_string());
        params.push(self.procedure.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InterfaceManagementReq value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InterfaceManagementReq {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub procedure: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InterfaceManagementReq".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "procedure" => intermediate_rep.procedure.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InterfaceManagementReq".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InterfaceManagementReq {
            procedure: intermediate_rep.procedure.into_iter().next().ok_or("procedure missing in InterfaceManagementReq".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InterfaceManagementReq> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<InterfaceManagementReq>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InterfaceManagementReq>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for InterfaceManagementReq - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<InterfaceManagementReq> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InterfaceManagementReq as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into InterfaceManagementReq - {}",
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
pub struct RefreshWorkerReq {
    #[serde(rename = "workerUniqueId")]
    pub worker_unique_id: uuid::Uuid,

    #[serde(rename = "f1Address")]
    pub f1_address: models::TransportAddress,

    #[serde(rename = "connectedAmfs")]
    pub connected_amfs: Vec<String>,

    #[serde(rename = "connectedDus")]
    pub connected_dus: Vec<String>,

}

impl RefreshWorkerReq {
    pub fn new(worker_unique_id: uuid::Uuid, f1_address: models::TransportAddress, connected_amfs: Vec<String>, connected_dus: Vec<String>, ) -> RefreshWorkerReq {
        RefreshWorkerReq {
            worker_unique_id: worker_unique_id,
            f1_address: f1_address,
            connected_amfs: connected_amfs,
            connected_dus: connected_dus,
        }
    }
}

/// Converts the RefreshWorkerReq value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RefreshWorkerReq {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping workerUniqueId in query parameter serialization

        // Skipping f1Address in query parameter serialization


        params.push("connectedAmfs".to_string());
        params.push(self.connected_amfs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());


        params.push("connectedDus".to_string());
        params.push(self.connected_dus.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RefreshWorkerReq value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RefreshWorkerReq {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub worker_unique_id: Vec<uuid::Uuid>,
            pub f1_address: Vec<models::TransportAddress>,
            pub connected_amfs: Vec<Vec<String>>,
            pub connected_dus: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RefreshWorkerReq".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "workerUniqueId" => intermediate_rep.worker_unique_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "f1Address" => intermediate_rep.f1_address.push(<models::TransportAddress as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "connectedAmfs" => return std::result::Result::Err("Parsing a container in this style is not supported in RefreshWorkerReq".to_string()),
                    "connectedDus" => return std::result::Result::Err("Parsing a container in this style is not supported in RefreshWorkerReq".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing RefreshWorkerReq".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RefreshWorkerReq {
            worker_unique_id: intermediate_rep.worker_unique_id.into_iter().next().ok_or("workerUniqueId missing in RefreshWorkerReq".to_string())?,
            f1_address: intermediate_rep.f1_address.into_iter().next().ok_or("f1Address missing in RefreshWorkerReq".to_string())?,
            connected_amfs: intermediate_rep.connected_amfs.into_iter().next().ok_or("connectedAmfs missing in RefreshWorkerReq".to_string())?,
            connected_dus: intermediate_rep.connected_dus.into_iter().next().ok_or("connectedDus missing in RefreshWorkerReq".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RefreshWorkerReq> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RefreshWorkerReq>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RefreshWorkerReq>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RefreshWorkerReq - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RefreshWorkerReq> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RefreshWorkerReq as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RefreshWorkerReq - {}",
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
pub struct RefreshWorkerRsp {
    #[serde(rename = "amfAddresses")]
    pub amf_addresses: Vec<models::TransportAddress>,

}

impl RefreshWorkerRsp {
    pub fn new(amf_addresses: Vec<models::TransportAddress>, ) -> RefreshWorkerRsp {
        RefreshWorkerRsp {
            amf_addresses: amf_addresses,
        }
    }
}

/// Converts the RefreshWorkerRsp value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RefreshWorkerRsp {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping amfAddresses in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RefreshWorkerRsp value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RefreshWorkerRsp {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub amf_addresses: Vec<Vec<models::TransportAddress>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RefreshWorkerRsp".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "amfAddresses" => return std::result::Result::Err("Parsing a container in this style is not supported in RefreshWorkerRsp".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing RefreshWorkerRsp".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RefreshWorkerRsp {
            amf_addresses: intermediate_rep.amf_addresses.into_iter().next().ok_or("amfAddresses missing in RefreshWorkerRsp".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RefreshWorkerRsp> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RefreshWorkerRsp>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RefreshWorkerRsp>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RefreshWorkerRsp - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RefreshWorkerRsp> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RefreshWorkerRsp as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RefreshWorkerRsp - {}",
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
pub struct TransportAddress {
    #[serde(rename = "host")]
    pub host: String,

    #[serde(rename = "port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<u16>,

}

impl TransportAddress {
    pub fn new(host: String, ) -> TransportAddress {
        TransportAddress {
            host: host,
            port: None,
        }
    }
}

/// Converts the TransportAddress value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TransportAddress {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("host".to_string());
        params.push(self.host.to_string());


        if let Some(ref port) = self.port {
            params.push("port".to_string());
            params.push(port.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TransportAddress value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TransportAddress {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub host: Vec<String>,
            pub port: Vec<u16>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TransportAddress".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "host" => intermediate_rep.host.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "port" => intermediate_rep.port.push(<u16 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TransportAddress".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TransportAddress {
            host: intermediate_rep.host.into_iter().next().ok_or("host missing in TransportAddress".to_string())?,
            port: intermediate_rep.port.into_iter().next(),
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

