#![cfg(target_os="android")]
#![allow(non_snake_case)]
use std::ffi::{CString, CStr};
use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jstring};



// Logger for Android LogCat - use error!() to print
#[macro_use] extern crate log;
extern crate android_logger;

mod android;
use android::logCatPrinter;

mod server;
use server::run_app;

mod controllers {
    pub mod index;
}

//#[cfg(target_os = "android")]
#[no_mangle]
pub unsafe extern fn Java_com_example_android_MainActivity_hello(env: JNIEnv, _: JObject, j_recipient: JString) -> jstring {
    let recipient = CString::from(
        CStr::from_ptr(
            env.get_string(j_recipient).unwrap().as_ptr()
        )
    );

    //let output = env.new_string("Novooo!! - http://cehum.ilch.uminho.pt".to_owned() + recipient.to_str().unwrap()).unwrap();
    let output = env.new_string("Okkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk".to_owned()).unwrap();

    logCatPrinter();

    error!("{}", "111111111111111111111 Teste Error ;)");

    //let native_activity = ndk_glue::native_activity();

    run_app();

    output.into_inner()
}


/*#[cfg(not(target_os = "android"))]
fn main() {
    run_app();
}*/

