#![allow(deprecated)]

/// 5.21 Device Type
///
/// The following table lists the type of device from which the impression originated.
///
/// OpenRTB version 2.2 of the specification added distinct values for Mobile and Tablet. It is
/// recommended that any bidder adding support for 2.2 treat a value of 1 as an acceptable alias of
/// 4 & 5.
///
/// This OpenRTB table has values derived from the Inventory Quality Guidelines (IQG). Practitioners
/// should keep in sync with updates to the IQG values.
#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i8)]
pub enum DeviceType {
    /// Mobile/Tablet
    #[deprecated(since = "0.1.0", note = "Please use the Phone or Tablet variant instead")]
    Mobile = 1,
    /// Personal Computer
    PersonalComputer,
    /// Connected TV
    ConnectedTv,
    /// Phone
    Phone,
    /// Tablet
    Tablet,
    /// Connected Device
    ConnectedDevice,
    /// Set Top Box
    SetTopBox,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<DeviceType>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<DeviceType> = serde_json::from_str(json)?;
        assert_eq!(e1, vec![DeviceType::Mobile, DeviceType::PersonalComputer]);
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
