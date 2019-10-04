use crate::api_key::ApiKey;

#[derive(Debug)]
pub struct ApiVersion {
    api_key: ApiKey,
    min_version: i16,
    max_version: i16,
}

impl ApiVersion {
    pub fn new(api_key: ApiKey, min_version: i16, max_version: i16) -> Self {
        ApiVersion {
            api_key,
            min_version,
            max_version,
        }
    }
}
