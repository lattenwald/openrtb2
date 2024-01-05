/// 3.2.1 [`BidRequest#at`], 3.2.12 [`Deal#at`]
///
/// Auction type, where 1 = First Price, 2 = Second Price Plus. Exchange-specific auction types can
/// be defined using values greater than 500.
///
/// [`BidRequest#at`]: ./struct.BidRequest.html#structfield.at
/// [`Deal#at`]: ./struct.Deal.html#structfield.at
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AuctionType {
    FirstPrice,
    SecondPricePlus,
    ExchangeSpecific(i32),
}

impl serde::Serialize for AuctionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(match self {
            Self::FirstPrice => 1,
            Self::SecondPricePlus => 2,
            Self::ExchangeSpecific(value) => *value,
        })
    }
}

impl<'de> serde::Deserialize<'de> for AuctionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match i32::deserialize(deserializer)? {
            1 => Ok(Self::FirstPrice),
            2 => Ok(Self::SecondPricePlus),
            value if value > 500 => Ok(Self::ExchangeSpecific(value)),
            value => Err(serde::de::Error::custom(format!(
                "invalid value: {}, expected 1 or 2 or greater than 500",
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
        assert!(serde_json::from_str::<AuctionType>("3").is_err());
        assert!(serde_json::from_str::<AuctionType>("500").is_err());

        let json = "[1,2,501]";
        let e1: Vec<AuctionType> = serde_json::from_str(json)?;
        assert_eq!(serde_json::to_string(&e1)?, json);
        assert_eq!(
            e1,
            vec![
                AuctionType::FirstPrice,
                AuctionType::SecondPricePlus,
                AuctionType::ExchangeSpecific(501)
            ]
        );

        Ok(())
    }
}
