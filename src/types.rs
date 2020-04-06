#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetricPayload {
    pub page_id: String,
    pub test_id: String,
    pub region: String,
    pub performance_score: i16,
    pub first_contentful_paint: i32,
    pub first_cpu_idle: i32,
    pub first_meaningful_paint: i32,
    pub interactive: i32,
    pub max_potential_fid: i32,
    pub speed_index: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ActionType {
    PushMetric,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SseMessage<T> {
    pub sub: String,
    pub event: String,
    pub action: ActionType,
    pub payload: T,
}