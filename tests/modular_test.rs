use rustnetworking::{modular::{inverse}, big_num::{BigUint, Digit}};

#[test]
fn test_inverse_of_3_mod_7() -> Result<(),String> {
    let x: BigUint = Digit::_3.into();
    let modulus: BigUint = Digit::_7.into();
    let res = inverse(x, modulus)?;

    assert_eq!(BigUint::from(Digit::_5), res);
    Ok(())
}