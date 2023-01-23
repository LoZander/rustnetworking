#![feature(test)]

extern crate test;
use test::Bencher;
use rustnetworking::{rsa};

#[test]
fn test_keygen_doesnt_give_err() -> Result<(),String> {
    rsa::keygen(2048)?;
    Ok(())
}

#[test]
fn test_decrypted_cipher_gives_original_plaintext() -> Result<(),String> {
    let plaintext = format!("this is a test");
    let plaintext_bytes = dbg!(plaintext.clone().into_bytes());

    let (pk,sk) = rsa::keygen(2048)?;
    let cipher = dbg!(rsa::encrypt(plaintext_bytes, pk));

    let res_bytes = dbg!(rsa::decrypt(cipher, sk)?);
    let res = String::from_utf8(res_bytes).map_err(|x|x.to_string())?;

    assert_eq!(plaintext, res);
    Ok(())
}

#[bench]
fn bench_generation(b: &mut Bencher) {
    b.iter(|| rsa::keygen(2048))
}