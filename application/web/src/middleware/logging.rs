use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use log::info;
use std::future::{ready, Ready};

/// ログ関連のミドルウェア
pub struct Logging;

impl<S, B> Transform<S, ServiceRequest> for Logging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddleware { service }))
    }
}

pub struct LoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_owned();
        let method = req.method().to_string();
        info!("API開始: {}:{}", method, path);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            info!("API終了: {}:{}", method, path);
            Ok(res)
        })
    }
}
