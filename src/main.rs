extern crate hyper;
extern crate tokio_core;

use hyper::Client;
use hyper::Uri;
use std::io::Read;
use tokio_core::reactor::Core;

fn main() {
    let bstr = ByteStr::new("http://web.mta.info/status/serviceStatus.txt");
    let uri = Uri::new(bstr).unwrap();
    let core = Core::new().unwrap();
    let handle = core.handle();
    let mut xml = String::new();

    let req = Client::new(&handle)
        .get(uri)
        .send()
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();

    println!("{} {}", xml, req);
}
