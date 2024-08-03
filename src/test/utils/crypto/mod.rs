use crate::utils::crypto::{aes256_cbc_decrypt, aes256_cbc_encrypt};

#[test]
fn aes_test(){
    let key = "1234567890tkltktqVdTstvuhlZHTest";
    let data = "hello world";
    let a = aes256_cbc_encrypt(data, key).unwrap();
    println!("{}",a);
    println!("{}",aes256_cbc_decrypt(&a,key).unwrap());
}