use chrono::NaiveDate;

use bolt_proto_derive::*;

use crate::error::*;

mod conversions;

pub(crate) const MARKER: u8 = 0xB1;
pub(crate) const SIGNATURE: u8 = 0x44;

// TODO: Decide on a final API for all the Date/Time stuff - it seems like we're just providing an alternative to
//   chrono's types. Should we force users to use chrono types and hide the wrapper types?
#[derive(Debug, Clone, Hash, Eq, PartialEq, Signature, Marker, Serialize, Deserialize)]
pub struct Date {
    pub(crate) days_since_epoch: i64,
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::sync::{Arc, Mutex};

    use bytes::Bytes;

    use crate::serialization::*;
    use crate::value::integer::{MARKER_INT_16, MARKER_INT_32};

    use super::*;

    #[test]
    fn get_marker() {
        let date = Date::from(NaiveDate::from_ymd(2020, 1, 1));
        assert_eq!(date.get_marker().unwrap(), MARKER);
    }

    #[test]
    fn try_into_bytes() {
        let date = Date::from(NaiveDate::from_ymd(1901, 12, 31));
        assert_eq!(
            date.try_into_bytes().unwrap(),
            Bytes::from_static(&[MARKER, SIGNATURE, MARKER_INT_16, 0x9E, 0xFA])
        );
    }

    #[test]
    fn try_from_bytes() {
        let past_date = Date::from(NaiveDate::from_ymd(1901, 12, 31));
        let past_bytes = &[MARKER_INT_16, 0x9E, 0xFA];
        let future_date = Date::from(NaiveDate::from_ymd(3000, 5, 23));
        let future_bytes = &[MARKER_INT_32, 0x00, 0x05, 0xBE, 0x16];
        assert_eq!(
            Date::try_from(Arc::new(Mutex::new(Bytes::from_static(past_bytes)))).unwrap(),
            past_date
        );
        assert_eq!(
            Date::try_from(Arc::new(Mutex::new(Bytes::from_static(future_bytes)))).unwrap(),
            future_date
        );
    }
}
