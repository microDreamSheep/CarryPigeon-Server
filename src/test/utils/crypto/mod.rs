use crate::utils::crypto::{
    aes256_cbc_decrypt, aes256_cbc_encrypt, generate_rsa_key, rsa_decrypt, rsa_encrypt,
};

#[test]
fn aes_test() {
    let key = "1234567890tkltktqVdTstvuhlZHTest";
    let data = "hello world";
    let a = aes256_cbc_encrypt(data, key).unwrap();
    println!("{}", a);
    println!("{}", aes256_cbc_decrypt(&a, key).unwrap());
}

#[test]
fn rsa_test() {
    let (pub_key, pri_key) = generate_rsa_key();
    let data = "hello world";
    let a = rsa_encrypt(&pub_key, data);
    println!("{}", a);
    println!("{}", rsa_decrypt(&pri_key, &a));
}
