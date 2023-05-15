#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct SenderData<T = serde_json::Value> {
    pub url: String,
    pub header: serde_json::Value,
    pub payload: T,
    pub from: String,
    #[serde(default)]
    pub error: serde_json::Value,
    #[serde(rename = "type")]
    pub message_type: String,
}
