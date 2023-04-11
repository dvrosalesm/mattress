use anyhow::bail;
use futures::{future::BoxFuture, Future};
use serde::Deserialize;
use std::collections::HashMap;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    POST,
    GET,
}

impl HttpMethod {
    pub fn from_str(method: &str) -> anyhow::Result<HttpMethod> {
        match method {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            _ => bail!("HTTP method not recognized {method}"),
        }
    }
}

type HandlerArgs = (Context, HttpRequest);
type HandlerResult = HttpResponse;

pub struct Handler {
    func: Box<dyn Fn(HandlerArgs) -> BoxFuture<'static, HandlerResult> + Send + Sync + 'static>,
}

impl Handler {
    fn new<P>(raw_func: fn(Context, HttpRequest) -> P) -> Handler
    where
        P: Future<Output = HandlerResult> + Send + 'static,
    {
        Handler {
            func: Box::new(move |(context, request)| Box::pin(raw_func(context, request))),
        }
    }

    pub async fn call(&self, args: HandlerArgs) -> HandlerResult {
        (self.func)(args).await
    }
}

pub struct Router {
    handlers: HashMap<HttpMethod, HashMap<String, Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn add_handler<P>(
        mut self,
        method: HttpMethod,
        name: &str,
        fun: fn(Context, HttpRequest) -> P,
    ) -> Self
    where
        P: Future<Output = HandlerResult> + Send + 'static,
    {
        if !self.handlers.contains_key(&method) {
            self.handlers.insert(method.clone(), HashMap::new());
        }

        if let Some(method_routes) = self.handlers.get_mut(&method) {
            method_routes.insert(name.to_string(), Handler::new(fun));
        }

        self
    }

    pub fn get(&self, method: HttpMethod, name: &str) -> anyhow::Result<&Handler> {
        if let Some(method_routes) = self.handlers.get(&method) {
            let name_key = format!("/{}", name).to_string();
            if let Some(handler) = method_routes.get(&name_key) {
                return Ok(handler);
            }
        }

        bail!("route not found {name}")
    }
}

pub fn deser<'de, T: Deserialize<'de>>(raw: &'de [u8]) -> RpcResult<T> {
    serde_json::from_slice(raw).map_err(|e| RpcError::Deser(format!("{}", e)))
}
