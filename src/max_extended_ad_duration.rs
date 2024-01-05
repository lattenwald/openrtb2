/// 3.2.7 [`Video#maxextended`], 3.2.8 [`Audio#maxextended`]
///
/// Maximum extended ad duration if extension is allowed. If blank or 0, extension is not allowed.
/// If -1, extension is allowed, and there is no time limit imposed. If greater than 0, then the
/// value represents the number of seconds of extended play supported beyond the maxduration value.
///
/// [`Video#maxextended`]: ./struct.Video.html#structfield.maxextended
/// [`Audio#maxextended`]: ./struct.Audio.html#structfield.maxextended
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MaxExtendedAdDuration {
    NoLimit,
    NotAllowed,
    Specific(i32),
}

impl serde::Serialize for MaxExtendedAdDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(match self {
            Self::NoLimit => -1,
            Self::NotAllowed => 0,
            Self::Specific(value) => *value,
        })
    }
}

impl<'de> serde::Deserialize<'de> for MaxExtendedAdDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match i32::deserialize(deserializer)? {
            -1 => Ok(Self::NoLimit),
            0 => Ok(Self::NotAllowed),
            value if value > 0 => Ok(Self::Specific(value)),
            value => Err(serde::de::Error::custom(format!(
                "invalid value: {}, expected -1 or 0 or greater than 0",
                value
            ))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<MaxExtendedAdDuration>("-2").is_err());

        let json = "[-1,0,1]";
        let e1: Vec<MaxExtendedAdDuration> = serde_json::from_str(json)?;
        assert_eq!(serde_json::to_string(&e1)?, json);
        assert_eq!(
            e1,
            vec![
                MaxExtendedAdDuration::NoLimit,
                MaxExtendedAdDuration::NotAllowed,
                MaxExtendedAdDuration::Specific(1),
            ]
        );

        Ok(())
    }
}
