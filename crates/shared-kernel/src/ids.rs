use std::fmt;

use uuid::Uuid;

/// Declares a UUID-backed identifier newtype, so ids of different aggregates
/// can never be swapped.
macro_rules! uuid_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord,
            serde::Serialize, serde::Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            /// Generates a new random identifier.
            #[must_use]
            pub fn generate() -> Self {
                Self(Uuid::new_v4())
            }

            #[must_use]
            pub const fn from_uuid(uuid: Uuid) -> Self {
                Self(uuid)
            }

            #[must_use]
            pub const fn as_uuid(&self) -> &Uuid {
                &self.0
            }
        }

        impl From<$name> for Uuid {
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

uuid_id!(
    /// A managed server, reached through the agent it runs.
    ServerId
);

uuid_id!(
    /// A Compose project on one server.
    StackId
);

uuid_id!(
    /// An account that signs in to the central.
    UserId
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_ids_differ() {
        assert_ne!(ServerId::generate(), ServerId::generate());
    }

    #[test]
    fn round_trips_through_uuid() {
        let uuid = Uuid::new_v4();
        let id = ServerId::from_uuid(uuid);
        assert_eq!(Uuid::from(id), uuid);
        assert_eq!(id.to_string(), uuid.to_string());
    }
}
