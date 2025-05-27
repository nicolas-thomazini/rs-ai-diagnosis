use crate::mistral::mistral_chat;
use crate::Result;

pub struct Buddy;

impl Buddy {
    pub async fn init() -> Result<Self> {
        Ok(Self)
    }

    pub async fn chat(&self, prompt: &str) -> Result<String> {
        let response = mistral_chat(prompt).await?;
        Ok(response)
    }
}
