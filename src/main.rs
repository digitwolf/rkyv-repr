use rkyv::{Archive, Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Clone, Archive, Deserialize, Serialize, Eq, PartialEq, Hash, Debug)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug))]
#[archive(check_bytes)]
pub struct Payload {
    /// Version of the ScottyPayload schema
    pub version: u8,
    /// Enum used to identify the type of the request data so that the receiver knows how to parse it.
    pub request_type: MessageType,
    /// The raw payload data.
    pub data: Vec<u8>,
}

/// Enum used to identify the type of the request data so that the receiver knows how to parse it
#[derive(Clone, Copy, Archive, Deserialize, Serialize, Debug, Eq, PartialEq, Hash)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug))]
#[archive(check_bytes)]
#[repr(u8)]
pub enum MessageType {
    HttpRequest = 1,
    HttpResponse = 2,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::HttpRequest
    }
}


impl TryFrom<Vec<u8>> for Payload {
    type Error = ConversionError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let scotty_payload = rkyv::from_bytes::<Self>(&bytes.as_slice()).unwrap_or_else( |deserialize_error| {
            println!("Failed to deserialize: {:?}", deserialize_error);
            Payload{
                version: 0,
                request_type: MessageType::HttpResponse,
                data: bytes
            }
        });

        Ok(scotty_payload)
    }
}

impl TryFrom<Payload> for Vec<u8> {
    type Error = ConversionError;

    fn try_from(payload: Payload) -> Result<Self, Self::Error> {
        rkyv::to_bytes::<_, 256>(&payload)
            .map(|aligned_vec| aligned_vec.into_vec())
            .map_err(|serialize_error| {
                println!("Failed to serialize {:?}. {}", &payload, serialize_error);
                ConversionError::Serialize(serialize_error.to_string())
            })
    }
}


impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}



/// Errors for ScottyPayload (De)Serialization
#[derive(Debug, Error, Eq, PartialEq)]
pub enum ConversionError {
    /// Failure to deserialize.
    #[error("Failed to deserialize because {0}")]
    Deserialize(String),
    /// Failure to serialize.
    #[error("Failed to serialize because {0}")]
    Serialize(String),
}



fn main() {
    let payload = Payload {
        version: 222,
        request_type: Default::default(),
        data: vec![1,2,3,4,5,6,7,8,9,10],
    };

    println!("{:?}", Vec::<u8>::try_from(payload.clone()).unwrap());
    assert_eq!(Vec::<u8>::try_from(payload).unwrap(), Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 0, 244, 255, 255, 255, 10, 0, 0, 0, 222, 0, 0, 0]));

}
