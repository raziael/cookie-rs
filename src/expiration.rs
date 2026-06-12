use time::OffsetDateTime;

/// A cookie's expiration: either a date-time or session.
///
/// An `Expiration` is constructible with `Expiration::from()` via any of:
///
///   * `None` -> `Expiration::Session`
///   * `Some(OffsetDateTime)` -> `Expiration::DateTime`
///   * `OffsetDateTime` -> `Expiration::DateTime`
///
/// ```rust
/// use cookie::Expiration;
/// use time::OffsetDateTime;
///
/// let expires = Expiration::from(None);
/// assert_eq!(expires, Expiration::Session);
///
/// let now = OffsetDateTime::now_utc();
/// let expires = Expiration::from(now);
/// assert_eq!(expires, Expiration::DateTime(now));
///
/// let expires = Expiration::from(Some(now));
/// assert_eq!(expires, Expiration::DateTime(now));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Expiration {
    /// Expiration for a "permanent" cookie at a specific date-time.
    DateTime(OffsetDateTime),
    /// Expiration for a "session" cookie. Browsers define the notion of a
    /// "session" and will automatically expire session cookies when they deem
    /// the "session" to be over. This is typically, but need not be, when the
    /// browser is closed.
    Session,
}

impl Expiration {
    /// Returns `true` if `self` is an `Expiration::DateTime`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Expiration;
    /// use time::OffsetDateTime;
    ///
    /// let expires = Expiration::from(None);
    /// assert!(!expires.is_datetime());
    ///
    /// let expires = Expiration::from(OffsetDateTime::now_utc());
    /// assert!(expires.is_datetime());
    /// ```
    pub fn is_datetime(&self) -> bool {
        match self {
            Expiration::DateTime(_) => true,
            Expiration::Session => false
        }
    }

    /// Returns `true` if `self` is an `Expiration::Session`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Expiration;
    /// use time::OffsetDateTime;
    ///
    /// let expires = Expiration::from(None);
    /// assert!(expires.is_session());
    ///
    /// let expires = Expiration::from(OffsetDateTime::now_utc());
    /// assert!(!expires.is_session());
    /// ```
    pub fn is_session(&self) -> bool {
        match self {
            Expiration::DateTime(_) => false,
            Expiration::Session => true
        }
    }

    /// Returns the inner `OffsetDateTime` if `self` is a `DateTime`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Expiration;
    /// use time::OffsetDateTime;
    ///
    /// let expires = Expiration::from(None);
    /// assert!(expires.datetime().is_none());
    ///
    /// let now = OffsetDateTime::now_utc();
    /// let expires = Expiration::from(now);
    /// assert_eq!(expires.datetime(), Some(now));
    /// ```
    pub fn datetime(self) -> Option<OffsetDateTime> {
        match self {
            Expiration::Session => None,
            Expiration::DateTime(v) => Some(v)
        }
    }

    /// Applied `f` to the inner `OffsetDateTime` if `self` is a `DateTime` and
    /// returns the mapped `Expiration`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Expiration;
    /// use time::{OffsetDateTime, Duration};
    ///
    /// let now = OffsetDateTime::now_utc();
    /// let one_week = Duration::weeks(1);
    ///
    /// let expires = Expiration::from(now);
    /// assert_eq!(expires.map(|t| t + one_week).datetime(), Some(now + one_week));
    ///
    /// let expires = Expiration::from(None);
    /// assert_eq!(expires.map(|t| t + one_week).datetime(), None);
    /// ```
    pub fn map<F>(self, f: F) -> Self
        where F: FnOnce(OffsetDateTime) -> OffsetDateTime
    {
        match self {
            Expiration::Session => Expiration::Session,
            Expiration::DateTime(v) => Expiration::DateTime(f(v)),
        }
    }
}

// NOTE: the original `impl<T: Into<Option<OffsetDateTime>>> From<T> for Expiration`
// triggers a coherence error under Rust 1.80+ stricter orphan rules due to a
// hypothetical conflict with `time`'s internal `From<HourBase> for ...::Type`.
// Replaced with two specific impls that cover every call site cookie itself
// uses (`OffsetDateTime`, `Option<OffsetDateTime>`, and `None`-by-inference).
// — raziael/cookie-rs fork patch for mira

impl From<OffsetDateTime> for Expiration {
    fn from(value: OffsetDateTime) -> Self {
        Expiration::DateTime(value)
    }
}

impl From<Option<OffsetDateTime>> for Expiration {
    fn from(value: Option<OffsetDateTime>) -> Self {
        match value {
            Some(dt) => Expiration::DateTime(dt),
            None => Expiration::Session,
        }
    }
}
