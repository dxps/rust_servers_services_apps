use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest::HttpRequest, httprequest::Method, httprequest::Resource};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            Method::Get => match &req.resource {
                Resource::Path(s) => {
                    // Parse the URI.
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        // If the route begins with /api, invoke the Web server.
                        "api" => {
                            let rsp = WebServiceHandler::handle(&req);
                            let _ = rsp.send_response(stream);
                        }
                        // Else, invoke the static page handler.
                        _ => {
                            let rsp = StaticPageHandler::handle(&req);
                            let _ = rsp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                // For other (non GET) requests, return the 404.
                let rsp = PageNotFoundHandler::handle(&req);
                let _ = rsp.send_response(stream);
            }
        }
    }
}
