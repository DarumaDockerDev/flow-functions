use http_req::request;
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn run(user: String) -> String {
    let mut writer = Vec::new(); //container for body of a response
    let res = request::get("https://httpbin.org/get", &mut writer).unwrap();

    println!("Status: {} {}", res.status_code(), res.reason());
    println!("Headers {}", res.headers());
    String::from_utf8_lossy(&writer).to_string()
}
