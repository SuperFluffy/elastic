//! http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-thread-pool.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

use RequestParams;

pub fn get<'a>(client: &'a mut Client, req: RequestParams, base: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd = String::with_capacity(base.len() + 17 + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_cat/thread_pool");
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
