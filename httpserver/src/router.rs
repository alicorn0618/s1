use std::io::Write;

use http::{httprequest::HttpRequest, httpresponse::HttpResponse};

use crate::handler::{StaticPageHandler, WebServiceHandler};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            http::httprequest::Method::Get => match &req.resource {
                http::httprequest::Resource::Path(s) => {
                    let route: Vec<_> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp = WebServiceHandler::hanlde(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
        }
    }
}
