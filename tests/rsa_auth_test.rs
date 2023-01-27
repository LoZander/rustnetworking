use rustnetworking::{
    rsa::{
        authenticity::{self as auth, Signature, Verification},
        keygen, 
        confidentiality::{self as conf, Message}}};

#[test]
fn verification_of_correct_message_sign_pair_accepts() -> Result<(),String> {
    let (pk,sk) = keygen(2048)?;
    let m: Message = "This is a test".into();
    let s: Signature = auth::sign(m.clone(), sk)?;
    let v: Verification = auth::verify(m, s, pk);
    
    match v {
        Verification::Reject => Err("verification failed".into()),
        Verification::Accept => Ok(())
    }
}

#[test]
fn verification_of_message_modified_by_adversary_rejects() -> Result<(),String> {
    let (pk,sk) = keygen(2048)?;
    let m: Message = "This is a test, once again".into();
    let s: Signature = auth::sign(m, sk)?;
    let v: Verification = auth::verify("This is a different message injected by an adversary >:D", s, pk);

    match v {
        Verification::Accept => Err("verification didn't fail, even though the message didn't match the signature".into()),
        Verification::Reject => Ok(())
    }

}

#[test]
fn message_cannot_be_forged_so_verification_accepts() -> Result<(),String> {
    let (pk,sk) = keygen(2048)?;
    let real_message: Message = "This is an actual message".into();
    let s: Signature = auth::sign(real_message, sk)?;
    let forgery: Message = conf::encrypt(s.clone(), pk.clone());
    let v: Verification = auth::verify(forgery,s,pk);

    match v {
        Verification::Accept => Err("forgery was accepted by verify".into()),
        Verification::Reject => Ok(())
    }
}