use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Deserializer, Serialize,
};

#[derive(Deserialize, Clone, Debug, Default)]
pub struct InitVideoResponse {
    pub value: Values,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Values {
    pub uploadUrlsExpireAt: u64,
    pub video: String,
    pub uploadInstructions: Vec<UploadInstructions>,
    pub uploadToken: String,
    #[serde(default = "default_field_val")]
    pub captionsUploadUrl: String,
    #[serde(default = "default_field_val")]
    pub thumbnailUploadUrl: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UploadInstructions {
    pub lastByte: u64,
    pub firstByte: u64,
    pub uploadUrl: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VideoUploadStatus {
    owner: String,
    id: String,
    status: String,
}

impl VideoUploadStatus {
    pub fn owner(&self) -> &str {
        &self.owner
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn status(&self) -> &str {
        &self.status
    }
}

fn default_field_val() -> String {
    "".to_string()
}
