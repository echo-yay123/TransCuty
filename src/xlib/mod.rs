
extern crate x11;
use std::ffi::CString;
use std::ffi::CStr;
use std::mem;
use std::os::raw::*;
use std::ptr;
use x11::xlib::*;

pub struct GetText{}

impl GetText{


fn get_selection_text(display: *mut _XDisplay, window: &Window, property : &Atom) -> Option<String> {
 
    let mut actual_return_type: c_ulong =0;
    let mut actual_return_format: c_int = 0;
    let mut nitems_return: c_ulong = 0;
    let mut bytes_after_return : c_ulong = 0;
    let mut temp : c_ulong = 0;
    let mut prop_return = 0 as *mut u8;
    
    //Get selection text size
    unsafe{
        XGetWindowProperty(
        display, 
        *window, 
        *property, 
        0, 
        0, 
        false as i32, 
        AnyPropertyType as u64, 
        &mut actual_return_type, 
        &mut actual_return_format, 
        &mut nitems_return, 
        &mut bytes_after_return, 
        &mut prop_return)
    };
    unsafe{XFree(prop_return as *mut c_void);}

    let incr_str = CString::new("INCR").unwrap();
    let incr = unsafe {XInternAtom(display,incr_str.as_ptr(),false as i32)};
    
    //Error
    if actual_return_type == incr {
        println!("Something wrong, panic!");
        return None;
    }

    //println!("The context size is : {:?}", bytes_after_return);
 
    unsafe{
        XGetWindowProperty(
            display, 
            *window, 
            *property, 
            0, 
            bytes_after_return as i64,
            false as i32,
            AnyPropertyType as u64,
            &mut actual_return_type,
            &mut actual_return_format,
            &mut nitems_return, 
            &mut temp, 
            &mut prop_return)
    };
    
    let text = unsafe { String::from_utf8_lossy(CStr::from_ptr(prop_return as *const i8).to_bytes()).to_string() };
    //println!("THE TEXT IS : {:?}",text);
    //println!("The context is :{:?}", prop_return);
    unsafe{
        XFree(prop_return as *mut c_void);
        XDeleteProperty(display, *window,*property);
    }
    //Return selection text
    Some(text)
}


pub fn convert_selection() -> Result<String,()> {
    
    //Open display connection.
    let display : *mut _XDisplay = unsafe {XOpenDisplay(ptr::null())};
    if display.is_null() {
        panic!("XopenDisplay failed!");
    }
    let screen = unsafe{XDefaultScreen(display)};
    let root = unsafe{XRootWindow(display, screen)};
 
    //Get selection owner   
    //let clipboard = CString::new("CLIPBOARD").unwrap();
    let clipboard = CString::new("PRIMARY").unwrap();
    let clipboard_atom = unsafe { XInternAtom(display, clipboard.as_ptr(), false as i32) };

    let _selection_owner = unsafe { XGetSelectionOwner(display, clipboard_atom) };
    //println!("Owner of {:#?} is : {:?}", clipboard, selection_owner);
      
    //Create virtual target window
    let target_window = unsafe{XCreateSimpleWindow(display, root,-10,-10,1,1,0,0,0)};
    let target = CString::new("ECHO").unwrap();
    let target_property = unsafe{XInternAtom(display, target.as_ptr(), false as i32)};

    //Store selected text in target window property
    let property = CString::new("UTF8_STRING").unwrap();
    let property_atom = unsafe { XInternAtom(display, property.as_ptr(), false as i32) };
    let _result = unsafe{XConvertSelection(display,clipboard_atom,property_atom,target_property,target_window,CurrentTime)};
    
    
    //Wait for trigger event, if return None convertion fails, else get the text from target window property and output.
    unsafe{
        loop{  
            let mut event: XEvent = mem::MaybeUninit::uninit().assume_init();
            XNextEvent(display, &mut event);
        
            if let x11::xlib::SelectionNotify = event.get_type(){
                
                let sev: XSelectionEvent = event.selection;
                if sev.property == 0 {
                    println!("Convertion failed!");
                }
                else {
                    //Get selection text
                    let result = Self::get_selection_text(display,&target_window,&target_property);
                    //Output text
                    match result {
                        None => return Err(()),
                        Some(text) => return Ok(text),
                    }
                }
                
                //Break loop
                break;
            }
     
        }
    }
    
    unsafe{XCloseDisplay(display)};
    return Err(());
}

}