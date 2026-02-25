use serde::{Deserialize, Serialize};

/// The internal payload of our license token.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LicensePayload {
    pub lines_enabled: bool,
    pub vectors_enabled: bool,
    pub allowed_points_regex: Option<Vec<String>>,
    pub allowed_traces_regex: Option<Vec<String>>,
    pub allowed_graphs_regex: Option<Vec<String>>,

    // Cryptographic Decoding: These values are only valid if the license is verified.
    // If someone bypasses the check, these remain default/garbage.
    pub vector_scale_factor: f32,
    pub default_point_size: f32,

    // A timestamp or expiration could go here
    pub issued_at: u64,
}

impl Default for LicensePayload {
    fn default() -> Self {
        Self {
            lines_enabled: false,
            vectors_enabled: false,
            allowed_points_regex: None,
            allowed_traces_regex: None,
            allowed_graphs_regex: None,
            vector_scale_factor: 0.0, // Invalid scale
            default_point_size: 0.0,  // Invalid size
            issued_at: 0,
        }
    }
}

/// A serialized License Token containing the base64 payload and its base64 signature.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LicenseToken {
    pub payload_b64: String,
    pub signature_b64: String,
}
