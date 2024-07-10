/// 3.2.13 Object: Site
///
/// This object should be included if the ad supported content is a website as opposed to a
/// non-browser application. A bid request must not contain both a Site and an App object. At a
/// minimum, it is useful to provide a site ID or page URL, but this is not strictly required.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Site<'a> {
    /// string; recommended
    /// Exchange-specific site ID.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub id: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Site name (may be aliased at the publisher’s request).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub name: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Domain of the site (e.g., “mysite.foo.com”).
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<std::borrow::Cow<'a, str>>,

    /// string array
    /// Array of IAB content categories of the site. Refer to List 5.1.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<std::borrow::Cow<'a, str>>>,

    /// string array
    /// Array of IAB content categories that describe the current section of the site. Refer to
    /// List 5.1.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<std::borrow::Cow<'a, str>>>,

    /// string array
    /// Array of IAB content categories that describe the current page or view of the site. Refer
    /// to List 5.1.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<std::borrow::Cow<'a, str>>>,

    /// string
    /// URL of the page where the impression will be shown.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub page: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Referrer URL that caused navigation to the current page.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<std::borrow::Cow<'a, str>>,

    /// string
    /// Search string that caused navigation to the current page.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub search: Option<std::borrow::Cow<'a, str>>,

    /// integer
    /// Indicates if the site has been programmed to optimize layout when viewed on mobile devices,
    /// where 0 = no, 1 = yes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<json_ext::Flag>,

    /// integer
    /// Indicates if the site has a privacy policy, where 0 = no, 1 = yes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<json_ext::Flag>,

    /// object
    /// Details about the Publisher (Section 3.2.15) of the site.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<crate::Publisher<'a>>,

    /// object
    /// Details about the Content (Section 3.2.16) within the site.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub content: Option<crate::Content<'a>>,

    /// string
    /// Comma separated list of keywords about the site.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<std::borrow::Cow<'a, str>>,

    /// object
    /// Placeholder for exchange-specific extensions to OpenRTB.
    // #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    // pub ext: Option<json_ext::Object<'a>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Value>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = "{}";
        let o1 = Site::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Site>(json)?);

        Ok(())
    }
}
