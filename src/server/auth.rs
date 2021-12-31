use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use actix_web::dev::{Service, Transform, ServiceRequest, ServiceResponse};
use futures::Future;
use futures::future::{ok, Ready};
use crate::logger::Logger;

pub struct TestMiddleware;
pub struct TestMiddlewareProcess<S> { service: Rc<RefCell<S>> }

impl<S: 'static, B> Transform<S, ServiceRequest> for TestMiddleware
    where S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
          S::Future: 'static,
          B: 'static
{
    //type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = TestMiddlewareProcess<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(TestMiddlewareProcess { service: Rc::new(RefCell::new(service)) })
    }
}

impl<S, B> Service<ServiceRequest> for TestMiddlewareProcess<S>
    where S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
          S::Future: 'static,
          B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = S::Future;//Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let logger = Rc::new(Logger::new(Some("TestMiddleware")));
        logger.debug("task call from middleware start.");

        // Todo: Write Token Validator Task (2021/12/30 23:54)

        self.service.call(req)
        // Box::pin(async move {
        //     let res = serv.call(req).await?;
        //     println!("headers: {:?}", res.headers());
        //     logger.debug("cage task call from middleware response");
        //
        //     Ok(res)
        // })
    }
}