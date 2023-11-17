
use TransCuty::{TranslateInfo, Request, TranslatedResult};
use reqwest::blocking::Client;
//use std::io;

//#![cfg_attr(not(feature = "xlib"), allow(dead_code))]
//#![cfg_attr(not(feature = "xlib"), allow(unused_imports))]



extern crate rdev;
use rdev::{listen, EventType, Button};
use std::sync::mpsc::channel;
use std::thread;

pub mod xlib;
use xlib::GetText;

fn main (){

        // Spawn new thread because listen blocks
        let (schan, rchan) = channel();
        let _listener = thread::spawn(move || {
            listen(move |event| {
                schan
                    .send(event)
                    .unwrap_or_else(|e| println!("Could not send event {:?}", e));
            })
            .expect("Could not listen");
        });
        
        //Left mouse button release trigger the translation function
        for event in rchan.iter() {    
            if let EventType::ButtonRelease(Button::Left) = event.event_type  {    
                //Select the PRIMARY X11 selection 
                //Converty to property of a virtual target window
                //Then print the selected text on screen.
                match GetText::convert_selection() {
                    Ok(text) => {
                        //Print the selected text on screen
                        println!("Please input original text:");
                        println!("{:?}",text.clone());
                        println!("^~^@_@^~^@_@^~^@_@^~^@_@^~^@_@^~^@_@^~^@_@^~^@_@^~^@_@^~^@_@^~^");
                        
                        //Convert text format according to Baidu API's request
                        let trans_info = TranslateInfo::build(&text);
                        
                        //Build a http client send request and get response
                        let client = Client::new();
                        let resp = Request::get_response(&trans_info, &client).unwrap();
                        
                        //Convert and show result from Baidu server's json response
                        let translated_result = TranslatedResult::show_result(resp.clone());
                        
                        //Output
                        match translated_result {
                            Ok(()) => {
                                println!("Yep!");
                            }
                            _ => {
                                println!("Error!");
                            }
                        }
                    },
                        
                    Err(()) => {
                        println!("Error!");
                    }
                }
            }    
        }       
}





