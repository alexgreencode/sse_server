
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, http::{Method, header}, HttpResponse};
use futures::future::{ok, Ready, Either};

use crate::helpers::{get_sub_from_query, get_token_from_query};
use crate::token::check;

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseMessage<T, E> {
    pub is_success: bool,
    #[serde(default)]
    pub error_code: E,
    pub payload: T,
}

pub struct Auth {
    public_routes: Vec<&'static str>,
}

impl Auth {
    pub fn init(public_routes: Vec<&'static str>) -> Auth
    {
        Auth {
            public_routes,
        }
    }
}

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(
          AuthMiddleware {
            service,
            public_routes: self.public_routes.clone(),
          }
        )
    }
}

pub struct AuthMiddleware<S> {
    service: S,
    public_routes: Vec<&'static str>,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let path = req.path().clone();
        println!("About to check token ===================> ... {}", path);

        let method = req.method().clone();
        if self.public_routes.contains(&path) || method == Method::OPTIONS {
            println!("Test call ===================> OK");
            return Either::Left(self.service.call(req));
        }

        let query_str = req.query_string();
        let login = get_sub_from_query(query_str);
        let token = get_token_from_query(query_str);

        if login == "" || token == "" {
            return Either::Right(ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .body("Unauthorized")
                        .into_body()
            )));
        }

        let is_success = match check(Some(login), &token) {
            Ok(_) => {
                true
            },
            Err(_) => {
                false
            }
        };

        println!(":::::::::::::::::::::::> Token checking result: {}", is_success);
        if is_success {
            return Either::Left(self.service.call(req));
        } else {
            return Either::Right(ok(req.into_response(
                HttpResponse::Unauthorized()
                    .body("Unauthorized")
                    .into_body()
            )));
        }
    }
}