use digest::Digest;

pub fn string_hashing(user_option: String,user_input: String)  {
    
    let user_opt=user_option.trim().to_string();

    if user_opt=="md5"
    {
     let hashed_output=md5::compute(&user_input);
     println!("md5 hash: {:?}",hashed_output);
    }
    else if user_opt=="sha224"
    {
      let hashed_output = sha2::Sha224::digest(&user_input.as_bytes());
      println!("Sha224 hash: {:?}",hashed_output);
    }
    else if user_opt=="sha256"
    {
      let hashed_output = sha2::Sha256::digest(&user_input.as_bytes());
      println!("Sha256 hash: {:?}",hashed_output);
    }
    else if user_opt=="sha384"
    {
      let hashed_output = sha2::Sha384::digest(&user_input.as_bytes());
      println!("Sha384 hash: {:?}",hashed_output);
    }
    else if user_opt=="sha512"
    {
      let hashed_output = sha2::Sha512::digest(&user_input.as_bytes());
      println!("Sha512 hash: {:?}",hashed_output);
    }
    else if user_opt=="sha3_224"
    {
      let hashed_output = sha3::Sha3_224::digest(&user_input.as_bytes());
      println!("Sha3_224 hash: {:?}",hashed_output);
    }
    else if user_opt=="sha3_256"
    {
      let hashed_output = sha3::Sha3_256::digest(&user_input.as_bytes());
      println!("Sha3_256 hash: {:?}",hashed_output);
    }
    else if user_opt=="sha3_384"
    {
      let hashed_output = sha3::Sha3_384::digest(&user_input.as_bytes());
      println!("Sha3_284 hash: {:?}",hashed_output);
    }
    else if user_opt=="sha3_512"
    {
      let hashed_output = sha3::Sha3_512::digest(&user_input.as_bytes());
      println!("Sha3_512 hash: {:?}",hashed_output);
    }
    else if user_opt=="gost94"
    {
      let hashed_output = gost94::Gost94Test::digest(&user_input.as_bytes());
      println!("Gost94 hash: {:?}",hashed_output);
    }
    else {
        println!("Invalid Option");
    }

}
