use p256::pkcs8::EncodePrivateKey;
use rand_core::OsRng; // requires 'getrandom' feature

pub async fn init() {
    let secret_key = p256::SecretKey::random(&mut OsRng);

    let secret_key_serialized = secret_key
        .to_pkcs8_encrypted_pem(&mut OsRng, "vja481xx", Default::default())
        .unwrap()
        .to_string();

    println!("{}", secret_key_serialized);
}
