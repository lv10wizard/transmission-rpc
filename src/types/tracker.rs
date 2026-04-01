use std::fmt::{self, Display};

use url::Url;
use serde::{Deserialize, Serialize, de::{Error, Visitor}};

/// Helper type representing a [tier] of tracker announce urls.
///
/// See: [`TrackerList`].
///
/// ### Example
///
/// In addition to manual construction, this type can be converted from any collection of
/// [`Url`]s (eg. `Vec<Url>`) by calling [`Into::into`].
///
/// ```rust
/// let tier = TrackerTier(vec![Url::parse("http://example.com/foo")?]);
/// let another_tier: TrackerTier = vec![Url::parse("https://announce.example.com:2010")?].into();
/// ```
///
/// [tier]: <https://www.bittorrent.org/beps/bep_0012.html>
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TrackerTier(pub Vec<Url>);

impl Display for TrackerTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let url_str: Vec<_> = self.0.iter().map(AsRef::<str>::as_ref).collect();
        write!(f, "{}", url_str.join("\n"))
    }
}

impl<I> From<I> for TrackerTier
where
    I: IntoIterator<Item = Url>,
{
    fn from(value: I) -> Self {
        Self(value.into_iter().collect())
    }
}

/// Represents a list of tracker announce urls ordered by their [tiers].
///
/// See:
///
/// * [`TorrentSetArgs::tracker_list`]
/// * [`Torrent::tracker_list`]
///
/// [tiers]: <https://www.bittorrent.org/beps/bep_0012.html>
/// [`TorrentSetArgs::tracker_list`]: super::TorrentSetArgs::tracker_list
/// [`Torrent::tracker_list`]: super::Torrent::tracker_list
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TrackerList(pub Vec<TrackerTier>);

impl Display for TrackerList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tiers: Vec<_> = self.0.iter()
            .map(ToString::to_string)
            .collect();
        write!(f, "{}", tiers.join("\n\n"))
    }
}

impl<I, T> From<I> for TrackerList
where
    I: IntoIterator<Item = T>,
    T: Into<TrackerTier>,
{
    fn from(value: I) -> Self {
        let tiers = value.into_iter()
            .map(Into::into)
            .collect();
        Self(tiers)
    }
}

impl Serialize for TrackerList {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        // I don't know if the trailing '\n' is strictly required by transmission, but the rpc
        // server returns `tracker_list` with a trailing '\n'.
        format!("{self}\n").serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TrackerList {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        deserializer.deserialize_str(TrackerListVisitor::default())
    }
}

#[derive(Default)]
struct TrackerListVisitor;

impl<'de> Visitor<'de> for TrackerListVisitor {
    type Value = TrackerList;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter,
            "a string of announce urls separated by '\\n',\
            tiers separated by \"\\n\\n\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let mut tracker_list: Vec<TrackerTier> = vec![];
        let mut tier = vec![];

        for tracker in v.split("\n") {
            // Tracker tiers are separated by a blank line.
            if tracker.is_empty() {
                tracker_list.push(tier.into());
                tier = vec![];
                continue
            }

            tier.push(Url::parse(tracker).map_err(Error::custom)?);
        }

        match tracker_list.is_empty() {
            true => Err(Error::custom("expected at least one '\\n'")),
            false => Ok(tracker_list.into())
        }
    }
}

#[cfg(test)]
mod serde_tests {
    use serde_json;
    use crate::types::Result;
    use super::*;

    #[test]
    fn tracker_list_serialize_single_tier_single_url() -> Result<()> {
        let list: TrackerList = vec![
            vec![Url::parse("https://one.example.com:1111")?],
        ].into();

        let serialized = serde_json::to_string(&list)?;
        println!("> {serialized}");
        assert_eq!(serialized, "\"https://one.example.com:1111/\\n\"");
        Ok(())
    }

    #[test]
    fn tracker_list_serialize_single_tier_multiple_url() -> Result<()> {
        let list: TrackerList = vec![
            vec![
                Url::parse("https://one.example.com:1111")?,
                Url::parse("https://two.example.com:2222")?,
                Url::parse("https://three.example.com:3333")?,
            ],
        ].into();

        let serialized = serde_json::to_string(&list)?;
        println!("> {serialized}");
        assert_eq!(serialized,
            "\"https://one.example.com:1111/\\n\
            https://two.example.com:2222/\\n\
            https://three.example.com:3333/\\n\"");
        Ok(())
    }

    #[test]
    fn tracker_list_serialize_multiple_tier_single_url() -> Result<()> {
        let list: TrackerList = vec![
            vec![Url::parse("https://one.example.com:1111")?],
            vec![Url::parse("https://two.example.com:2222")?],
            vec![Url::parse("https://three.example.com:3333")?],
            vec![Url::parse("https://four.example.com:4444")?],
        ].into();

        let serialized = serde_json::to_string(&list)?;
        println!("> {serialized}");
        assert_eq!(serialized,
            "\"https://one.example.com:1111/\\n\\n\
            \
            https://two.example.com:2222/\\n\\n\
            \
            https://three.example.com:3333/\\n\\n\
            \
            https://four.example.com:4444/\\n\"");
        Ok(())
    }

    #[test]
    fn tracker_list_serialize_multiple_tier_multiple_url() -> Result<()> {
        let list: TrackerList = vec![
            vec![
                Url::parse("https://one.example.com:1111")?,
                Url::parse("https://two.example.com:2222")?,
            ],
            vec![Url::parse("https://three.example.com:3333")?],
            vec![Url::parse("https://four.example.com:4444")?],
            vec![
                Url::parse("https://five.example.com:5555")?,
                Url::parse("https://six.example.com:6666")?,
                Url::parse("https://seven.example.com:7777")?,
            ],
        ].into();

        let serialized = serde_json::to_string(&list)?;
        println!("> {serialized}");
        assert_eq!(serialized,
            "\"https://one.example.com:1111/\\n\
            https://two.example.com:2222/\\n\\n\
            \
            https://three.example.com:3333/\\n\\n\
            \
            https://four.example.com:4444/\\n\\n\
            \
            https://five.example.com:5555/\\n\
            https://six.example.com:6666/\\n\
            https://seven.example.com:7777/\\n\"");
        Ok(())
    }

    #[test]
    fn tracker_list_deserialize_single_tier_single_url() -> Result<()> {
        let deserialized: TrackerList = serde_json::from_str(r#"
            "https://one.example.com:1111\n"
        "#)?;

        println!("< {:#?}", &deserialized);

        let expected: TrackerList = vec![vec![Url::parse("https://one.example.com:1111")?]].into();
        assert_eq!(deserialized, expected);
        Ok(())
    }

    #[test]
    fn tracker_list_deserialize_single_tier_multiple_url() -> Result<()> {
        let deserialized: TrackerList = serde_json::from_str(r#"
            "https://one.example.com:1111\nhttps://two.example.com:2222\n"
        "#)?;

        println!("< {:#?}", &deserialized);

        let expected: TrackerList = vec![
            vec![
                Url::parse("https://one.example.com:1111")?,
                Url::parse("https://two.example.com:2222")?,
            ]
        ]
        .into();
        assert_eq!(deserialized, expected);
        Ok(())
    }

    #[test]
    fn tracker_list_deserialize_multiple_tier_single_url() -> Result<()> {
        let deserialized: TrackerList = serde_json::from_str(r#"
            "https://one.example.com:1111\n\nhttps://foo.example.com/bar\n"
        "#)?;

        println!("< {:#?}", &deserialized);

        let expected: TrackerList = vec![
            vec![Url::parse("https://one.example.com:1111")?],
            vec![Url::parse("https://foo.example.com/bar")?],
        ]
        .into();
        assert_eq!(deserialized, expected);
        Ok(())
    }

    #[test]
    fn tracker_list_deserialize_multiple_tier_multiple_url() -> Result<()> {
        let deserialized: TrackerList = serde_json::from_str(r#"
            "https://one.example.com:1111\nhttps://two.example.com:2222\n\nhttps://foo.example.com/bar\n\nhttps://lorem.example.com/ipsum\n\nhttps://three.example.com:3333\nhttps://four.example.com:4444\nhttps://five.example.com:5555\n"
        "#)?;

        println!("< {:#?}", &deserialized);

        let expected: TrackerList = vec![
            vec![
                Url::parse("https://one.example.com:1111")?,
                Url::parse("https://two.example.com:2222")?,
            ],
            vec![Url::parse("https://foo.example.com/bar")?],
            vec![Url::parse("https://lorem.example.com/ipsum")?],
            vec![
                Url::parse("https://three.example.com:3333")?,
                Url::parse("https://four.example.com:4444")?,
                Url::parse("https://five.example.com:5555")?,
            ],
        ]
        .into();
        assert_eq!(deserialized, expected);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "relative URL without a base")]
    fn tracker_list_deserialize_malformed_url() {
        match serde_json::from_str::<TrackerList>(r#"
            "malformed"
        "#)
        {
            Ok(deserialized) => {
                println!("< {:#?}", &deserialized);
            },

            Err(err) => panic!("{err}"),
        }
    }

    #[test]
    #[should_panic(expected = "expected at least one '\\n'")]
    fn tracker_list_deserialize_malformed_tracker_list() {
        match serde_json::from_str::<TrackerList>(r#"
            "http://foo.bar.example.com:1234/announce"
        "#)
        {
            Ok(deserialized) => {
                println!("< {:#?}", &deserialized);
            },

            Err(err) => panic!("{err}"),
        }
    }
}
