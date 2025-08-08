pub struct TokenService {}

impl TokenService {
    pub fn validate_token(&self, token: &str) -> Result<(), String> {
        println!("TODO: evaluating token");
        Ok(())
    }
}
