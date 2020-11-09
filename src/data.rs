/// 3.2.21 Object: Data
///
/// The data and segment objects together allow additional data about the related object (e.g.,
/// user, content) to be specified. This data may be from multiple sources whether from the exchange
/// itself or third parties as specified by the id field. A bid request can mix data objects from
/// multiple providers. The specific data providers in use should be published by the exchange a
/// priori to its bidders.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Data<'a> {
    /// string
    /// Exchange-specific ID for the data provider.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Exchange-specific name for the data provider.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub name: Option<std::borrow::Cow<'a, str>>,

    /// object array
    /// Array of Segment (Section 3.2.22) objects that contain the actual data values.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub segment: Option<Vec<crate::Segment<'a>>>,

    /// object
    /// Placeholder for exchange-specific extensions to OpenRTB.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<json_ext::Ext>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = "{}";
        let o1 = Data::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Data>(json)?);

        Ok(())
    }
}
