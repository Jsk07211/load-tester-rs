#[derive(Clone, Debug)]
pub enum PayloadSpec {
    // Bytes(Vec<u8>),
    Json(serde_json::Value),
    // Template { fields: Vec<(String, FieldGen)> },
}

pub enum FieldGen {
    Static(Vec<u8>),
    RandomInt { min: i64, max: i64 },
    Uuid,
    Timestamp,
}
