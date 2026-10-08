use std::fmt;

/// Wraps a sensitive value so that it never reaches a log or a response.
///
/// `Debug` and `Display` print `[redacted]`; reading the value takes an
/// explicit [`Redacted::expose`] call.
///
/// `Redacted` implements no `Serialize`, so it cannot leak through a JSON body:
///
/// ```compile_fail
/// fn assert_serialize<T: serde::Serialize>(_: &T) {}
///
/// assert_serialize(&shared_kernel::Redacted::new(String::from("secret")));
/// ```
///
/// The same helper accepts the bare value, so the failure above comes from
/// `Redacted` alone:
///
/// ```
/// fn assert_serialize<T: serde::Serialize>(_: &T) {}
///
/// assert_serialize(&String::from("secret"));
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct Redacted<T>(T);

impl<T> Redacted<T> {
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    /// Returns the wrapped value; every call site is a place where the
    /// secret is used on purpose.
    pub const fn expose(&self) -> &T {
        &self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Redacted<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> fmt::Debug for Redacted<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[redacted]")
    }
}

impl<T> fmt::Display for Redacted<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[redacted]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_hides_the_value() {
        let formatted = format!("{:?}", Redacted::new("secret"));
        assert!(!formatted.contains("secret"));
        assert_eq!(formatted, "[redacted]");
    }

    #[test]
    fn display_hides_the_value() {
        let formatted = format!("{}", Redacted::new("secret"));
        assert!(!formatted.contains("secret"));
    }

    #[test]
    fn debug_hides_the_value_inside_a_struct() {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct Enrolment {
            token: Redacted<String>,
        }
        let enrolment = Enrolment {
            token: Redacted::new(String::from("secret")),
        };
        assert!(!format!("{enrolment:?}").contains("secret"));
    }

    #[test]
    fn expose_returns_the_value() {
        assert_eq!(*Redacted::new(42).expose(), 42);
    }
}
