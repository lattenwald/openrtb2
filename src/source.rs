/// 3.2.2 Object: Source
///
/// This object describes the nature and behavior of the entity that is the source of the bid
/// request upstream from the exchange. The primary purpose of this object is to define post-auction
/// or upstream decisioning when the exchange itself does not control the final decision. A common
/// example of this is header bidding, but it can also apply to upstream server entities such as
/// another RTB exchange, a mediation platform, or an ad server combines direct campaigns with 3rd
/// party demand in decisioning.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Source<'a> {
    /// integer; recommended
    /// Entity responsible for the final impression sale decision, where 0 = exchange, 1 = upstream
    /// source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fd: Option<SaleDecision>,

    /// string; recommended
    /// Transaction ID that must be common across all participants in this bid request (e.g.,
    /// potentially multiple exchanges).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub tid: Option<std::borrow::Cow<'a, str>>,

    /// string; recommended
    /// Payment ID chain string containing embedded syntax described in the TAG Payment ID Protocol
    /// v1.0.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub pchain: Option<std::borrow::Cow<'a, str>>,

    /// object
    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<json_ext::Object<'a>>,
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i8)]
pub enum SaleDecision {
    Exchange,
    Upstream,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = "{}";
        let o1 = Source::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Source>(json)?);

        Ok(())
    }
}
