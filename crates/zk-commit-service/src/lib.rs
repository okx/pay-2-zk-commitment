#![feature(generic_const_exprs)]

pub mod controllers;
pub mod error;
pub mod integration;
pub mod service;
pub mod types;

#[derive(Debug, Clone)]
pub struct AppState {}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }
}

#[macro_export]
macro_rules! build_app {
    (@app $app_state:ident) => {
        App::new()
        .app_data($app_state.clone())
        .service(root::health)
    };
    (@wrap) => {
        web::scope("/api/v1/wrap")
        .service(wrap::wrap_groth16)
    };
    (@append $base:expr $(, $service:expr)*) => {
        $base$(.service($service))*
    };
    (gateway, $app_state:ident) => {
        build_app!(
            @append
            build_app!(@app $app_state),
            build_app!(@wrap)
        )
    };
}
