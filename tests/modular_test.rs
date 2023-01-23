use rustnetworking::{modular::{inverse}, big_num::BigUint};

#[test]
fn test_inverse_of_3_mod_7() -> Result<(),String> {
    let x = BigUint::from_i32(3)?;
    let modulus = BigUint::from_i32(7)?;
    let res = inverse(x, modulus)?;

    assert_eq!(BigUint::from_i32(5)?, res);
    Ok(())
}