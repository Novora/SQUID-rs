use std::time::{SystemTime, UNIX_EPOCH};

/// Struct representing the version 0 implementation of the SQUID ID generation system.
/// 
/// The `SQUIDv0` struct generates unique, sortable IDs using a combination of the device UUID,
/// the current timestamp, and an internal counter to handle rapid successive calls.
///
/// # Warning
///
/// **This v0 implementation must not be used in applications where privacy of the device is critical, such as exposing the ID to the internet, because the device UUID is exposed.**
pub struct SQUIDv0 {
    device_uuid: String,
    counter: usize,
    last_timestamp: u128,
}

impl SQUIDv0 {
    /// Creates a new `SQUIDv0` instance.
    ///
    /// The device UUID is retrieved using the `machine_uuid` library. If the retrieval fails,
    /// a default UUID of "00000000-0000-0000-0000-000000000000" is used.
    ///
    /// # Examples
    ///
    /// ```
    /// use sortable_quick_unique_id::{SQUID, SQUIDv0};
    ///
    /// let squid = SQUIDv0::new(None);
    /// ```
     #[must_use]
    pub fn new(device_uuid: Option<&str>) -> Self {
        let uuid = device_uuid.map_or_else(
            || machine_uuid::get().unwrap_or_else(|_| "00000000-0000-0000-0000-000000000000".to_string()),
            |s| s.to_string(),
        );
        Self {
            device_uuid: uuid,
            counter: 0,
            last_timestamp: 0,
        }
    }

    /// Generates a unique ID.
    ///
    /// The ID is a combination of the device UUID, the current timestamp in milliseconds,
    /// and a counter to ensure uniqueness for multiple IDs generated within the same millisecond.
    ///
    /// # How it works
    ///
    /// 1. The current timestamp is retrieved in milliseconds since the Unix epoch.
    /// 2. If the timestamp is the same as the last generated timestamp, the counter is incremented.
    /// 3. If the timestamp is different, the counter is reset to 0.
    /// 4. The ID is formatted as "DeviceUUID-Timestamp-Counter".
    ///
    /// # Panics
    /// The generate function could panic if there was a unsigned integer overflow of the timestamp,
    /// which is highly unlikely to happen for a very long time(several billon years or more).
    ///
    /// # Examples
    ///
    /// ```
    /// use sortable_quick_unique_id::{SQUID, SQUIDv0};
    ///
    /// let mut squid = SQUIDv0::new(None);
    /// let id = squid.generate();
    /// println!("Generated ID: {}", id);
    /// ```
    pub fn generate(&mut self) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        if timestamp == self.last_timestamp {
            self.counter += 1;
        } else {
            self.counter = 0;
            self.last_timestamp = timestamp;
        }

        let counter_str = format!("{:04}", self.counter);

        // Format: DeviceUUID-Timestamp-Counter
        format!("{}-{}-{}", self.device_uuid, timestamp, counter_str)
    }
}

#[cfg(test)]
mod tests {
    use super::SQUIDv0;
    use std::collections::HashSet;

    #[test]
    fn test_generate_unique_ids() {
        let mut squid = SQUIDv0::new(None);
        let mut generated_ids = HashSet::new();
        let total_ids = 1_000_000;

        for _ in 0..total_ids {
            let id = squid.generate();
            assert!(!generated_ids.contains(&id), "Duplicate ID found: {}", id);
            generated_ids.insert(id);
        }

        assert_eq!(generated_ids.len(), total_ids, "Not all IDs are unique");
    }
}
