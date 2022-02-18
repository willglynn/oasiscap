use serde::de::Error;
use serde::{Deserialize, Deserializer};

/// Deserialize optional URLs, being generous on failure.
///
/// Valid URLs must always be passed through without modification. Invalid URLs may be
/// second-guessed, e.g. `"http://"` ought to be treated as `None`, and `"www.fema.org"` ought to
/// be treated as `"http://www.fema.org"`.
pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<url::Url>, D::Error>
where
    D: Deserializer<'de>,
{
    if let Some(string) = <Option<std::borrow::Cow<str>>>::deserialize(deserializer)? {
        parse(&string).map_err(|_| D::Error::custom(format!("invalid URL: {:?}", string)))
    } else {
        Ok(None)
    }
}

pub(crate) fn parse(string: &str) -> Result<Option<url::Url>, ()> {
    if let Ok(url) = url::Url::parse(string) {
        Ok(Some(url))
    } else if let Some(url) = assume_url_is_missing_http(string) {
        Ok(Some(url))
    } else if treat_url_as_missing(string) {
        Ok(None)
    } else {
        Err(())
    }
}

fn treat_url_as_missing(url: &str) -> bool {
    matches!(url, "http://" | "https://")
}

fn assume_url_is_missing_http(url: &str) -> Option<url::Url> {
    // See if it's a domain name
    let dotted_part: Vec<&str> = url.split('/').next().unwrap().split('.').collect();

    if !dotted_part
        .iter()
        .all(|part| part.chars().all(|c| c.is_ascii_alphanumeric()))
    {
        // Not a domain name
        return None;
    }

    match dotted_part.last().cloned() {
        Some("com") | Some("org") | Some("net") | Some("gov") | Some("us") => {
            // Could plausibly be an URL missing a scheme
            // Assume they meant http:// and see if that would be a valid URL
            url::Url::parse(&format!("http://{}", url)).ok()
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    fn de(
        str: &'static str,
    ) -> Result<
        Option<url::Url>,
        <&mut serde_test::Deserializer<'static> as serde::Deserializer>::Error,
    > {
        use serde_test::*;
        let tokens = &[Token::Some, Token::Str(str)];
        let mut de = Deserializer::new(tokens);
        super::deserialize(&mut de)
    }

    #[test]
    fn test_discard() {
        assert_eq!(de("http://").unwrap(), None);
        assert_eq!(de("https://").unwrap(), None);
    }

    #[test]
    fn test_fixup() {
        assert_eq!(
            de("www.moalerts.mo.gov").unwrap(),
            Some(url::Url::parse("http://www.moalerts.mo.gov").unwrap())
        );
        assert_eq!(
            de("www.missingkids.org").unwrap(),
            Some(url::Url::parse("http://www.missingkids.org").unwrap())
        );
        assert_eq!(
            de("www.missingkids.com").unwrap(),
            Some(url::Url::parse("http://www.missingkids.com").unwrap())
        );
        assert_eq!(
            de("www.ready.nj.gov").unwrap(),
            Some(url::Url::parse("http://www.ready.nj.gov").unwrap())
        );
        assert_eq!(
            de("www.miamidade.gov").unwrap(),
            Some(url::Url::parse("http://www.miamidade.gov").unwrap())
        );
        assert_eq!(
            de("newmexico.gov").unwrap(),
            Some(url::Url::parse("http://newmexico.gov").unwrap())
        );
        assert_eq!(
            de("www.alachuacounty.us/em").unwrap(),
            Some(url::Url::parse("http://www.alachuacounty.us/em").unwrap())
        );
        assert_eq!(
            de("www.fema.org").unwrap(),
            Some(url::Url::parse("http://www.fema.org").unwrap())
        );
    }
}
