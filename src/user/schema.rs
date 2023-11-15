use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(ToSchema, Deserialize, Serialize, Debug)]
#[salvo(schema(example = json!({"name": "bob the cat", "id": 0})))]
pub(super) struct UserInfo {
    id: u64,
    name: String,
}
