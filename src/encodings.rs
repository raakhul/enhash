use ascii_converter::*;
use std::str;
use std::*;

pub fn string_encode(user_option: String,user_input: String) -> (String, String) {

    let mut encoded_output = String::new();

    let user_opt=user_option.trim().to_string();

    if user_opt=="base64"
    {
        encoded_output=base64::encode(&user_input);
    }
    else if user_opt=="url"
    {
        encoded_output=urlencoding::encode(&user_input);
    }
    else if user_opt=="html"
    {
        encoded_output=htmlescape::encode_attribute(&user_input);
    }
    else if user_opt=="binary"
    {
        let vector_type_input=string_to_binary(&user_input).unwrap();
       
        encoded_output=u32_type_conversion_string(vector_type_input);
    }
    else if user_opt=="decimal"
    {
       let vector_type_input=string_to_decimals(&user_input).unwrap();
       
       encoded_output=u8_type_conversion_string(vector_type_input);
    }
    else if user_opt=="hex"
    {
        encoded_output=hex::encode(&user_input);
    }
    else {
        println!("Invalid Option");
    }
    
    return (user_opt, encoded_output);

}


pub fn numeric_decode(user_option: String,user_input: String) -> (String, String) {

    let mut decoded_output = String::new();

    let mut user_opt=user_option.trim().to_string();

    if user_opt=="base64"
    {
        let base64_converted = match base64::decode(&user_input) {
            Ok(output) => output,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
     
        decoded_output = match str::from_utf8(&base64_converted) {
            Ok(v) => v.to_string(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    }
    else if user_opt=="url"
    {
        decoded_output=urlencoding::decode(&user_input).unwrap();
    }
    else if user_opt=="html"
    {
        decoded_output=htmlescape::decode_html(&user_input).unwrap();   
    }
    
    else if user_opt=="binary"
    {
        //let mut new_string = String::new();
        
        /*
        let vector_type_input=string_to_binary(&user_input).unwrap();
        println!("vector type {:?}",vector_type_input);
        let u8type = &user_input.as_bytes();
        println!("u8 {:?}",u8type) ;
        decoded_output=binary_to_string(&vector_type_input).unwrap();
        println!("string type {}",decoded_output);
        */

        /*
        let mut numbers = String::new();

        io::stdin()
            .read_line(&mut numbers)
            .ok()
            .expect("read error");
    
        let numbers: Vec<i32> = numbers
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();
    
        for num in numbers {
            println!("{}", num);
        }
        */

    }
    /*
    else if user_opt=="decimal"
    {
       let vector_type_input=decimals_to_string(&user_input);.unwrap();
       
       decoded_output=u8_type_conversion_string(vector_type_input);
    }
   
    else if user_opt=="hex"
    {
        decoded_output=hex::decode(&user_input).unwrap();
    }
     */
    else {
        println!("Invalid Option");
    }

    user_opt="String".to_string();

    return (user_opt, decoded_output);

}


fn u8_type_conversion_string (vector_type_input:Vec<u8>) -> String {

    let mut thevalue = String::new();

    for i in 0..vector_type_input.len()
    {
        thevalue=thevalue+&vector_type_input[i].to_string();
    }

    return thevalue;
}

fn u32_type_conversion_string (vector_type_input:Vec<u32>) -> String {

    let mut thevalue = String::new();

    for i in 0..vector_type_input.len()
    {
        thevalue=thevalue+&vector_type_input[i].to_string();
       
    }

    return thevalue;
}
