extern crate ed25519_dalek;
extern crate rand;

use ed25519_dalek::Keypair;
use ed25519_dalek::Signer;
use rand::rngs::OsRng;

use base64::encode;

lazy_static! {
    static ref KEYPAIR: Keypair = Keypair::generate(&mut OsRng {});
}

pub fn with_appended_sig(message: &str) -> String {
    format!("{}\n\n{}", message, signature(message))
}

pub fn public_key() -> String {
    encode(KEYPAIR.public.to_bytes())
}

fn signature(message: &str) -> String {
    encode(KEYPAIR.sign(message.as_bytes()))
}

#[cfg(test)]
mod tests {

    use super::*;
    use base64::decode;
    use ed25519_dalek::Signature;
    use std::convert::TryFrom;

    #[test]
    fn test_verify_signature() {
        let message = "hello world";
        let b64_message_signature = signature(message);
        let sig_bytes: Vec<u8> = decode(b64_message_signature).unwrap();
        let sig: Signature = Signature::try_from(sig_bytes.as_slice()).unwrap();
        assert!(KEYPAIR.verify(message.as_bytes(), &sig).is_ok());
    }

    #[test]
    fn test_verify_format() {
        let message = "hello world";
        let actual = with_appended_sig(message);
        let expected = format!("hello world\n\n{}", signature(message));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_public_key() {
        let expected = encode(KEYPAIR.public.to_bytes());
        assert_eq!(super::public_key(), expected);
    }
}
