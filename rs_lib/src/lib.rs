use ed25519::signature::SignerMut;
use ed25519_dalek::{
  Signature, SigningKey, VerifyingKey, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH,
};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

// https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/skip.html
#[wasm_bindgen]
pub struct Ed25519Keypair {
  #[wasm_bindgen(skip)]
  pub public_key: Vec<u8>,
  #[wasm_bindgen(skip)]
  pub private_key: Vec<u8>,
}

#[wasm_bindgen]
pub fn generate_key_pair() -> Result<Ed25519Keypair, JsValue> {
  let mut csprng = OsRng;
  let keypair = SigningKey::generate(&mut csprng);
  let secret = keypair.as_bytes().to_vec().into();
  let public = keypair.verifying_key().as_bytes().to_vec().into();
  Ok(Ed25519Keypair {
    public_key: public,
    private_key: secret,
  })
}

#[wasm_bindgen]
pub fn from_private_key(private_key: &[u8]) -> Result<Ed25519Keypair, JsValue> {
  if private_key.len() != SECRET_KEY_LENGTH {
    return Err(JsValue::from_str(
      "Invalid privateKey length, must be 32 bytes",
    ));
  }
  let mut pk = [0; 32];
  pk.copy_from_slice(private_key);

  let secret_key = SigningKey::from_bytes(&pk);
  let public_key = secret_key.verifying_key();

  // https://rustwasm.github.io/wasm-bindgen/reference/types/boxed-slices.html
  // let secret: Box<[u8]> = secret_key.as_bytes().to_vec().into();
  // let public: Box<[u8]> = public_key.as_bytes().to_vec().into();
  Ok(Ed25519Keypair {
    public_key: public_key.as_bytes().to_vec(),
    private_key: secret_key.as_bytes().to_vec(),
  })
}

#[wasm_bindgen]
pub fn sign(private_key: &[u8], message: &[u8]) -> Result<Vec<u8>, JsValue> {
  if private_key.len() != SECRET_KEY_LENGTH {
    return Err(JsValue::from_str(
      "Invalid privateKey length, must be 32 bytes",
    ));
  }
  let mut pk = [0; 32];
  pk.copy_from_slice(&private_key);
  let mut key_pair = SigningKey::from_bytes(&pk);
  let signature = key_pair.sign(message);
  Ok(signature.to_bytes().to_vec())
}

#[wasm_bindgen]
pub fn verify(
  public_key: &[u8],
  message: &[u8],
  signature_buffer: &[u8],
) -> Result<bool, JsValue> {
  let signature = Signature::from_slice(signature_buffer)
    .map_err(|e| JsValue::from_str(&format!("Invalid signature {}", e)))?;
  if public_key.len() != PUBLIC_KEY_LENGTH {
    return Err(JsValue::from_str(
      "Invalid privateKey length, must be 32 bytes",
    ));
  }
  let mut pk = [0; 32];
  pk.copy_from_slice(&public_key);

  let pub_key = VerifyingKey::from_bytes(&pk)
    .map_err(|e| JsValue::from_str(&format!("Invalid public key {}", e)))?;

  Ok(pub_key.verify_strict(message, &signature).is_ok())
}
