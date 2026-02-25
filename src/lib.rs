use serde::{Deserialize, Serialize};

/// The internal payload of our license token.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LicensePayload {
    pub points_enabled: bool,
    pub lines_enabled: bool,
    pub vectors_enabled: bool,
    pub allowed_points_regex: Option<String>,
    
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
            points_enabled: false,
            lines_enabled: false,
            vectors_enabled: false,
            allowed_points_regex: None,
            vector_scale_factor: 0.0, // Invalid scale 
            default_point_size: 0.0, // Invalid size
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
