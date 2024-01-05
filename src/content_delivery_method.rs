/// 5.15 Content Delivery Methods
///
/// The following table lists the various options for the delivery of video or audio content.
#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i8)]
pub enum ContentDeliveryMethod {
    /// Streaming
    Streaming = 1,
    /// Progressive
    Progressive,
    /// Download
    Download,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<ContentDeliveryMethod>("-1").is_err());

        let json = "[1,2]";
        let e1: Vec<ContentDeliveryMethod> = serde_json::from_str(json)?;
        assert_eq!(
            e1,
            vec![ContentDeliveryMethod::Streaming, ContentDeliveryMethod::Progressive]
        );
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
