//! Tmux version checks

use std::cmp::Ordering;

/// Represents Tmux version.
///
/// If for some reason `tmux -V` does not work, or can't parse the output
/// Will return [Version::Max].
#[derive(Debug, PartialEq)]
pub enum TmuxVersion {
    Max,
    Version(usize, usize),
}

impl From<Option<&str>> for TmuxVersion {
    fn from(value: Option<&str>) -> Self {
        if value.is_none() {
            return Self::Max;
        }
        let value = value.unwrap();
        let re = regex::Regex::new(r"(?P<major>\d+)\.(?P<minor>\d+)").unwrap();
        let captures = re.captures(value);

        if captures.is_none() {
            return Self::Max;
        }

        let captures = captures.unwrap();
        let major_match = captures.name("major");
        let minor_match = captures.name("minor");

        let major: usize = match major_match {
            None => 0,
            Some(s) => s.as_str().parse().unwrap_or(0),
        };

        let minor = match minor_match {
            None => 0,
            Some(s) => s.as_str().parse().unwrap_or(0),
        };

        Self::Version(major, minor)
    }
}

impl PartialOrd for TmuxVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (TmuxVersion::Max, TmuxVersion::Max) => Some(Ordering::Equal),
            (TmuxVersion::Max, TmuxVersion::Version(_, _)) => Some(Ordering::Greater),
            (TmuxVersion::Version(_, _), TmuxVersion::Max) => Some(Ordering::Less),
            (TmuxVersion::Version(x1, x2), TmuxVersion::Version(y1, y2)) => {
                if x1 == y1 {
                    return x2.partial_cmp(y2);
                }
                return x1.partial_cmp(y1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TmuxVersion;
    #[test]
    fn test_version_strings_none() {
        let version: TmuxVersion = None.into();
        assert_eq!(version, TmuxVersion::Max);
    }

    #[test]
    fn test_version_strings() {
        let empty = Some("");
        let some_version = Some("tmux 3.3a");
        let next_version = Some("tmux next-3.4");

        assert_eq!(TmuxVersion::from(empty), TmuxVersion::Max);
        assert_eq!(TmuxVersion::from(some_version), TmuxVersion::Version(3, 3));
        assert_eq!(TmuxVersion::from(next_version), TmuxVersion::Version(3, 4));
    }

    #[test]
    fn test_version_compare() {
        assert!(TmuxVersion::Max > TmuxVersion::Version(2, 0));
        assert!(TmuxVersion::Version(3, 0) > TmuxVersion::Version(2, 900));
        assert!(TmuxVersion::Version(3, 1) < TmuxVersion::Version(3, 2));
        assert!(TmuxVersion::Version(3, 1) >= TmuxVersion::Version(3, 1));
    }
}
