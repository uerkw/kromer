#[derive(Debug, serde::Deserialize)]
pub struct LimitAndOffset {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}