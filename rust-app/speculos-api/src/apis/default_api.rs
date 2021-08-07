/*
 * Speculos API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::pin::Pin;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct DefaultApiClient<C: hyper::client::connect::Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> DefaultApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> DefaultApiClient<C> {
        DefaultApiClient {
            configuration,
        }
    }
}

pub trait DefaultApi {
    fn apdu_post(&self, apdu: crate::models::Apdu) -> Pin<Box<dyn Future<Output = Result<crate::models::Apdu, Error>>>>;
    fn automation_post(&self, body: serde_json::Value) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn button_button_post(&self, button: crate::models::ButtonName, button: crate::models::Button) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn events_delete(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn events_get(&self, stream: Option<bool>) -> Pin<Box<dyn Future<Output = Result<String, Error>>>>;
    fn finger_post(&self, finger: crate::models::Finger) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn screenshot_get(&self, ) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>>>>;
}

impl<C: hyper::client::connect::Connect>DefaultApi for DefaultApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn apdu_post(&self, apdu: crate::models::Apdu) -> Pin<Box<dyn Future<Output = Result<crate::models::Apdu, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/apdu".to_string())
        ;
        req = req.with_body_param(apdu);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn automation_post(&self, body: serde_json::Value) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/automation".to_string())
        ;
        req = req.with_body_param(body);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn button_button_post(&self, button_name: crate::models::ButtonName, button: crate::models::Button) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/button/{button}".to_string())
        ;
        req = req.with_path_param("button".to_string(), button_name.to_string());
        req = req.with_body_param(button);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn events_delete(&self, ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::DELETE, "/events".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn events_get(&self, stream: Option<bool>) -> Pin<Box<dyn Future<Output = Result<String, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/events".to_string())
        ;
        if let Some(ref s) = stream {
            let query_value = s.to_string();
            req = req.with_query_param("stream".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn finger_post(&self, finger: crate::models::Finger) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/finger".to_string())
        ;
        req = req.with_body_param(finger);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn screenshot_get(&self, ) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/screenshot".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

}
