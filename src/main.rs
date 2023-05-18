
use TransCuty::{TranslateInfo, Request, TranslatedResult};
use reqwest::blocking::Client;
use std::io;

fn main() {
   
        println!("Please input original text:");
        let mut text = String::new();
        match io::stdin().read_line(&mut text){
          Ok(_n) => {
                println!("^~^@_@^~^@_@^~^@_@^~^@_@^~^@_@^~^@_@^~^@_@^~^@_@^~^@_@^~^@_@^~^");
          }
          Err(error) =>{
                println!("{error}");
          }
        }
        
        let trans_info = TranslateInfo::build(&text);
        //println!("{:#?}",trans_info);
        
        let client = Client::new();
        let resp = Request::get_response(&trans_info, &client).unwrap();
        //println!("{:#?}",resp);

        let translated_result = TranslatedResult::show_result(resp.clone());
        match translated_result {
                Ok(()) => {
                        println!("Yep!");
                }
                _ => {
                        println!("Error!");
                }
        }
}
