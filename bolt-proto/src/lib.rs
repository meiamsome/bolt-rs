pub use message::Message;
pub use serialization::{Deserialize, Marker, Serialize, Signature};
pub use value::Value;

pub mod error;
pub mod message;
mod serialization;
pub mod value;

// TODO: Check for extraneous uses of String::from in message/value tests, since I removed some conversions from the
//   String wrapper type and now String::from refers to std::string::String::from instead of crate::value::String::from

// TODO: Replace any calls to integer max_value() with MAX (max_value has been deprecated)

// TODO: Remove '.value' from equality assertion for some value type tests

#[doc(hidden)]
#[macro_export]
macro_rules! impl_message_with_metadata {
    ($T:path) => {
        impl $T {
            pub fn new(
                metadata: ::std::collections::HashMap<::std::string::String, $crate::value::Value>,
            ) -> Self {
                Self { metadata }
            }

            pub fn metadata(
                &self,
            ) -> &::std::collections::HashMap<::std::string::String, $crate::value::Value> {
                &self.metadata
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_try_from_value {
    ($T:path, $V:ident) => {
        impl ::std::convert::TryFrom<$crate::Value> for $T {
            type Error = $crate::error::Error;

            fn try_from(value: $crate::Value) -> $crate::error::Result<Self> {
                match value {
                    $crate::Value::$V(inner) => Ok(inner),
                    _ => Err($crate::error::ConversionError::FromValue(value).into()),
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_try_from_message {
    ($T:path, $V:ident) => {
        impl ::std::convert::TryFrom<$crate::Message> for $T {
            type Error = $crate::error::Error;

            fn try_from(message: $crate::Message) -> $crate::error::Result<Self> {
                match message {
                    $crate::Message::$V(inner) => Ok(inner),
                    _ => Err($crate::error::ConversionError::FromMessage(message).into()),
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_empty_message_tests {
    ($T:ident) => {
        mod tests {
            use ::bytes::Bytes;
            use ::std::convert::TryFrom;
            use ::std::sync::{Arc, Mutex};

            use crate::serialization::*;

            use super::*;

            #[test]
            fn get_marker() {
                assert_eq!($T.get_marker().unwrap(), MARKER);
            }

            #[test]
            fn get_signature() {
                assert_eq!($T.get_signature(), SIGNATURE);
            }

            #[test]
            fn try_into_bytes() {
                let msg = $T;
                assert_eq!(
                    msg.try_into_bytes().unwrap(),
                    Bytes::from_static(&[MARKER, SIGNATURE])
                );
            }

            #[test]
            fn try_from_bytes() {
                let msg = $T;
                let msg_bytes = &[];
                assert_eq!(
                    $T::try_from(Arc::new(Mutex::new(Bytes::from_static(msg_bytes)))).unwrap(),
                    msg
                );
            }
        }
    };
}
