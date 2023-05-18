use tinyrand::{Rand, StdRand};
//use md5::Digest;


//User information from baidu
const APP_ID: &str = "A Baidu appid ";
const KEY: &str = "A Baidu key";
const BASE_URL :&str = "http://api.fanyi.baidu.com/api/trans/vip/translate?";


#[derive(Debug)]
pub struct TranslateInfo{
    q: String,//Text to be translated
    from: String,//Source Language 
    to: String,//Target Language
    appid: String, //APPID
    salt: String, //Random number or char
    key: String, //key
    signature: String,
}

impl TranslateInfo{
    
    fn generate_salt()->u32{
        let mut rand = StdRand::default();
        let s:u32 = rand.next_u32();
        s
    }
      
    pub fn build(text:&str)->TranslateInfo{
        //Generate a random number s
        let s:u32=Self::generate_salt();
        //Build 
        let mut trans_info=TranslateInfo{
            q: String::from(text),
            from: String::from("en"),
            to: String::from("zh"),
            appid: String::from(APP_ID),
            key: String::from(KEY),
            salt: s.to_string(),
            signature: String::new(),
        };
        
        let signature = trans_info.get_sign();
        trans_info.signature = String::from(signature);
        
        //Return
        trans_info
    }

    fn get_sign(&self) -> String{
        //Rules to generate signature, https://fanyi-api.baidu.com/doc/21
        let s1= String::from(&self.appid);
        let s2=String::from(&self.q);
        let s3=String::from(&self.salt);
        let s4 = String::from(&self.key);
        let signature = s1 + &s2 + &s3 + &s4;
        //println!("{signature}");
        
        //MD5 compute, return a [u8;16] structure
        let digest = md5::compute(&signature);
        //Convert digest to String type, string length=32 & all char lowercase
        let signature = digest.0.iter().map(|c| { format!("{:02X}", c).to_lowercase()}).collect();
        
        signature
    }
    
}

use reqwest::Url;
use reqwest::blocking::Client;
use std::io::Read;
pub struct Request{}

impl Request{
    
     pub fn get_response(trans_info: &TranslateInfo, client: &Client) -> Result<String, Box<dyn std::error::Error>>{
        
        let url = Request::make_url(
            BASE_URL,
            &[
                ("appid", &trans_info.appid),
                ("q", &trans_info.q ),
                ("from", & trans_info.from),
                ("to", & trans_info.to),
                ("salt", &trans_info.salt),
                ("sign", &trans_info.signature),
            ],
        )?;
        //println!("{:?}",url);
        let mut body = String::new();
        client
            .get(url)
            // .header(Connection::close())
            .send()?
            .read_to_string(&mut body)?;
    
        Ok(body)
        

    }

  
    
    fn make_url(url: &str, query: &[(&str, &str)]) -> Result<Url, Box<dyn std::error::Error>>{

        let mut url = Url::parse(url)?;
        url.query_pairs_mut().extend_pairs(query.iter());
        Ok(url)

    }
}

use serde::{Deserialize, Serialize};
use serde_json::{self};
//use serde_json::Result;
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct TranslatedResult{
    from: String,
    to: String, 
    trans_result: Vec<TransResult>,
    
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct TransResult {
    src: String,
    dst: String,
}

impl TranslatedResult {
  
    pub fn show_result(resp: String) -> serde_json::Result<()>{
        
        let v: TranslatedResult = serde_json::from_str(&resp)?;
        //println!("The text is : {:?}", v.trans_result[0].src);
        println!("The translation is : {:?}", v.trans_result[0].dst);

        Ok(())

    }
}


