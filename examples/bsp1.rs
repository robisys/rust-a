
extern crate curl;

use curl::http;

pub fn rbsp1(){

  let purl = "http://raspberrypi.fritz.box";

  let resp = http::handle()  
  .post(purl, "this is the body")
  .exec().unwrap();

  println!("code={}; headers={:?}; body={:?}",
    resp.get_code(), resp.get_headers(), resp.get_body());

}


pub fn rbsp2(){
  let purl = "http://raspberrypi.fritz.box";

  let resp = http::handle()
    .get(purl).exec().unwrap();

  println!("code={}; headers={:?}; body={:?}",
    resp.get_code(), resp.get_headers(), resp.get_body());
}


pub fn rbsp4() {
  let purl = "http://raspberrypi.fritz.box";

  let resp = http::handle().get(purl).exec().unwrap();
  println!("url: {}",purl);
  
  println!("code={}; headers={:?}; ",
    resp.get_code(), resp.get_headers());

}


pub fn main() {

 rbsp4();

rbsp2();
rbsp1();

}
