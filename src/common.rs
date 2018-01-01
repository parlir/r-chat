pub trait JsonSerialize {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        match serde_json::to_string(&self) {
            Ok(json) => Ok(json),
            Err(err) => Err(err)
        }
    }
}

