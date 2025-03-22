use napi_derive::napi;

fn hello() -> String {
  "hello".to_string()
}

#[napi]
pub fn hello_world() -> String {
  hello()
}
