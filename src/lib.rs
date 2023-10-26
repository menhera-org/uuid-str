
use std::str::FromStr;

/// A wrapper around a UUID string.
/// It uses a static 36 byte array to store the UUID string.
/// It allows for Copy and Clone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UuidStr {
    bytes: [u8; 36],
}

impl Default for UuidStr {
    fn default() -> Self {
        Self {
            bytes: *b"00000000-0000-0000-0000-000000000000",
        }
    }
}

impl UuidStr {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_v4() -> Self {
        let uuid = uuid::Uuid::new_v4();
        Self::from(&uuid)
    }

    pub fn bytes(&self) -> [u8; 36] {
        self.bytes
    }
}

impl AsRef<str> for UuidStr {
    fn as_ref(&self) -> &str {
        std::str::from_utf8(&self.bytes).unwrap()
    }
}

impl std::fmt::Display for UuidStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl FromStr for UuidStr {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = uuid::Uuid::parse_str(s)?;
        Ok(Self::from(&uuid))
    }
}

impl std::ops::Deref for UuidStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl std::convert::TryFrom<&str> for UuidStr {
    type Error = uuid::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

impl std::convert::TryFrom<String> for UuidStr {
    type Error = uuid::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(&s)
    }
}

impl AsRef<[u8]> for UuidStr {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl From<&uuid::Uuid> for UuidStr {
    fn from(uuid: &uuid::Uuid) -> Self {
        let str = uuid.as_hyphenated().to_string().to_ascii_lowercase();
        let bytes = str.as_bytes();
        assert_eq!(bytes.len(), 36);
        let mut uuid_str = Self::default();
        uuid_str.bytes.copy_from_slice(bytes);
        uuid_str
    }
}

impl From<uuid::Uuid> for UuidStr {
    fn from(uuid: uuid::Uuid) -> Self {
        Self::from(&uuid)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let default_uuid = UuidStr::default();
        let default_str: &str = default_uuid.as_ref();
        assert_eq!(default_str, "00000000-0000-0000-0000-000000000000");
    }

    #[test]
    fn v4() {
        let uuid = UuidStr::new_v4();
        let bytes: &[u8] = uuid.as_ref();
        assert_eq!(bytes.len(), 36);
        let maybe_uuid = uuid::Uuid::parse_str(uuid.as_ref());
        assert!(maybe_uuid.is_ok());
    }
}
