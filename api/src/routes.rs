pub mod v1;

#[derive(Debug, serde::Deserialize)]
struct LimitAndOffset {
    limit: Option<u64>,
    offset: Option<u64>,
}
