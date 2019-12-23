use std::convert::TryFrom;

use failure::Error;

use rust_bolt_macros::*;

use crate::bolt::value::BoltValue;
use crate::error::ValueError;

pub const SIGNATURE: u8 = 0x4E;

// TODO: Can't derive Signature et. al. if struct fields are Boxes, need to impl Serialize/Deserialize separately
//       and make the derive macro implement the traits separately :(

#[derive(Debug, Clone, Hash, Eq, PartialEq, Signature, Marker, Serialize, Deserialize)]
pub struct Node {
    node_identity: Box<BoltValue>,
    labels: Box<BoltValue>,
    properties: Box<BoltValue>,
}

// TODO: impl From<[Native Node type]> for Node

impl TryFrom<BoltValue> for Node {
    type Error = Error;

    fn try_from(value: BoltValue) -> Result<Self, Self::Error> {
        match value {
            BoltValue::Node(node) => Ok(node),
            _ => Err(ValueError::InvalidConversion(value).into()),
        }
    }
}
