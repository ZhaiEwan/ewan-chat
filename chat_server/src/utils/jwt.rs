use crate::{error::AppError, models::User};
use anyhow::Result;
use jwt_simple::prelude::*;
const JWT_ISS: &str = "chat_server";
const JWT_AUD: &str = "chat_web";

#[derive(Debug, Clone)]
pub(crate) struct EncodingKey(Ed25519KeyPair);
#[derive(Debug, Clone)]
pub(crate) struct DecodingKey(Ed25519PublicKey);

impl EncodingKey {
    pub(crate) fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }
    pub(crate) fn sign(&self, user: impl Into<User>) -> Result<String, AppError> {
        let claim = Claims::with_custom_claims(user.into(), Duration::from_hours(2));
        let claim = claim.with_audience(JWT_AUD).with_issuer(JWT_ISS);
        let token = self.0.sign(claim)?;
        Ok(token)
    }
}

impl DecodingKey {
    pub(crate) fn load(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }
    pub(crate) fn verify(&self, token: &str) -> Result<User, AppError> {
        let opt = VerificationOptions {
            allowed_issuers: Some(HashSet::from_strings(&[JWT_ISS])),
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUD])),
            ..Default::default()
        };
        let claim = self.0.verify_token::<User>(token, Some(opt))?;
        Ok(claim.custom)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    #[tokio::test]
    async fn crate_verify_token() -> Result<()> {
        let private_pem = include_str!("../../../private.pem");
        let public_pem = include_str!("../../../public.pem");

        let ek = EncodingKey::load(private_pem)?;
        let dk = DecodingKey::load(public_pem)?;

        let user = User {
            id: 1,
            fullname: "zhai".to_string(),
            password_hash: None,
            email: "zhai@gmail".to_string(),
            created_at: chrono::Utc::now(),
        };

        let token = ek.sign(user.clone())?;
        println!("token is :{}", token);
        let user2 = dk.verify(&token)?;
        assert_eq!(user, user2);
        Ok(())
    }
}
