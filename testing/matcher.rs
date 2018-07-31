use linker;

/// Matches a specific location in a solidity file.
pub trait LocationMatcher {
    /// Test if the given location matches.
    fn matches_location(&self, object: Option<&linker::Object>, function: Option<&str>) -> bool;
}

/// A default matcher.
#[derive(Debug, Clone, Copy)]
pub struct Matcher {
    path: Option<&'static str>,
    item: Option<&'static str>,
    function: Option<&'static str>,
}

impl Matcher {
    pub fn new() -> Self {
        Self {
            path: None,
            item: None,
            function: None,
        }
    }

    // Set the expected path.
    pub fn path(self, path: &'static str) -> Self {
        Self {
            path: Some(path),
            ..self
        }
    }

    // Set the expected item.
    pub fn item(self, item: &'static str) -> Self {
        Self {
            item: Some(item),
            ..self
        }
    }

    // Set the expected function.
    pub fn function(self, function: &'static str) -> Self {
        Self {
            function: Some(function),
            ..self
        }
    }
}

impl LocationMatcher for Matcher {
    fn matches_location(&self, object: Option<&linker::Object>, function: Option<&str>) -> bool {
        if let Some(expected_path) = self.path {
            if !object.map(|s| s.path == expected_path).unwrap_or(false) {
                return false;
            }
        }

        if let Some(expected_item) = self.item {
            if !object.map(|s| s.item == expected_item).unwrap_or(false) {
                return false;
            }
        }

        if let Some(expected_function) = self.function {
            if !function.map(|f| f == expected_function).unwrap_or(false) {
                return false;
            }
        }

        true
    }
}

impl LocationMatcher for &'static str {
    fn matches_location(&self, object: Option<&linker::Object>, function: Option<&str>) -> bool {
        let mut it = self.split(":");

        let matcher = Matcher::new();

        let first = match it.next() {
            Some(value) => value,
            None => return false,
        };

        let second = match it.next() {
            Some(value) => value,
            // only match function
            None => {
                return matcher.function(first).matches_location(object, function);
            }
        };

        let third = match it.next() {
            Some(value) => value,
            // match item + function
            None => {
                return matcher
                    .item(first)
                    .function(second)
                    .matches_location(object, function);
            }
        };

        matcher
            .path(first)
            .item(second)
            .function(third)
            .matches_location(object, function)
    }
}

#[cfg(test)]
mod tests {
    use super::LocationMatcher;
    use linker;

    #[test]
    fn test_with_str() {
        assert!("Test.sol:Test:foo".matches_location(
            Some(&linker::Object {
                path: "Test.sol".to_string(),
                item: "Test".to_string(),
            }),
            Some("foo"),
        ));

        assert!("Test:foo".matches_location(
            Some(&linker::Object {
                path: "Anything.sol".to_string(),
                item: "Test".to_string(),
            }),
            Some("foo"),
        ));

        assert!("foo".matches_location(
            Some(&linker::Object {
                path: "Anything.sol".to_string(),
                item: "Anything".to_string(),
            }),
            Some("foo"),
        ));

        assert!("foo".matches_location(None, Some("foo"),));
        assert!(!"foo".matches_location(None, None));
    }
}
