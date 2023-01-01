//! # Core Log functionality
//!
//! Log provides an easy way to log a message to the console with a simple, readable output format.
//!

// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// Implements [`Log`] to log a message to the console with a simple, readable output format.
///
/// # Arguments
///
/// * `session_id` - A string slice that holds a session ID. The session ID is a unique identifier for the current session. A random GUID (Globally Unique Identifier) is generated by default.
/// * `time` - A string slice that holds the timestamp in ISO 8601 format.
/// * `level` - A string slice that holds the level (INFO, WARN, ERROR, etc.).
/// * `component` - A string slice that holds the component name.
/// * `description` - A string slice that holds the description of the log message.
///
/// # Examples
/// ```
/// use mini_functions::log::Log;
///
/// let log = Log::new(
/// "12345678-1234-1234-1234-1234567890ab",
/// "2023-12-23T23:23:23.222222+00:00",
/// "INFO",
/// "SystemTrayEvent",
/// "Showing main window",
/// );
/// ```
///

#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Log {
    session_id: String,
    time: String,
    level: String,
    component: String,
    description: String,
}

impl Log {
    /// Create a new `Log` instance.
    /// # Arguments
    /// * `session_id` - A string slice that holds a session ID. The session ID is a unique identifier for the current session. A random GUID (Globally Unique Identifier) is generated by default.
    /// * `time` - A string slice that holds the timestamp in ISO 8601 format.
    /// * `level` - A string slice that holds the level (INFO, WARN, ERROR, etc.).
    /// * `component` - A string slice that holds the component name.
    /// * `description` - A string slice that holds the description of the log message.
    ///
    /// # Returns
    /// A new `Log` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions::log::Log;
    ///
    /// let log = Log::new(
    /// "12345678-1234-1234-1234-1234567890ab",
    /// "2023-12-23T23:23:23.222222+00:00",
    /// "INFO",
    /// "SystemTrayEvent",
    /// "Showing main window",
    /// );
    /// ```
    #[must_use]
    pub fn new(
        session_id: &str,
        time: &str,
        level: &str,
        component: &str,
        description: &str,
    ) -> Self {
        Self {
            session_id: session_id.to_string(),
            time: time.to_string(),
            level: level.to_string(),
            component: component.to_string(),
            description: description.to_string(),
        }
    }
    pub fn log(&self) {
        println!(
            "SessionID={} Timestamp={} Level={} Component={} Description=\"{}\"",
            self.session_id, self.time, self.level, self.component, self.description
        );
    }
}
