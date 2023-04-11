use mattress_interfaces::Mattress;
use router::{
    deser,
    HttpMethod::{self, GET, POST},
    Router,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::HttpClientSender;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

use jsonrpc::simple_http::{self, SimpleHttpTransport};
use jsonrpc::Client;

mod router;

const WAREHOUSE_ADDRESS: &str = "0xeFDCBBE9066857B8955C9eB663Ec69B5ed49474b";
const CHAIN_ID: u64 = 31;
const PROVIDER_URL: &str = "localhost:8545";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct MattressActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for MattressActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let path = &req.path[1..req.path.len()];
        let segments: Vec<&str> = path.trim_end_matches('/').split('/').collect();
        let method = HttpMethod::from_str(req.method.as_ref()).unwrap();

        let router = Router::new()
            .add_handler(GET, "/healthcheck", healthcheck)
            .add_handler(POST, "/deploy", deploy);

        if let Ok(handler) = router.get(method, segments[0]) {
            return Ok(handler.call((ctx.clone(), req.clone())).await);
        }

        Ok(HttpResponse {
            body: "Route not found".as_bytes().to_vec(),
            ..Default::default()
        })
    }
}

async fn healthcheck(_: Context, _: HttpRequest) -> HttpResponse {
    HttpResponse {
        body: "Ok".as_bytes().to_vec(),
        ..Default::default()
    }
}


async fn deploy(ctx: Context, req: HttpRequest) -> HttpResponse {
    let mattress: Mattress = deser(&req.body).unwrap();

    let client = HttpClientSender::new();
    let res = client.request(ctx, &wasmcloud_interface_httpclient::HttpRequest::post("http://localhost:8545"));
    
    HttpResponse {
        body: format!("{res}").as_bytes().to_vec(),
        ..Default::default()
    }
}
