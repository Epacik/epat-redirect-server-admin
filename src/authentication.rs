use std::pin::Pin;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use rbatis::crud::CRUD;
use crate::database::{RB, ApiKeysBlocked};
use crate::database::ApiKeys;

pub async fn validate_authorization(request: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, actix_web::Error> {

    let ip_result = request.headers().get("X-Real-IP");
    let mut ip = "";
    if ip_result.is_some() {
        ip = ip_result.unwrap().to_str().unwrap();
    }

    log::info!("authenticating\n{}\npeer ip address: {}",credentials.token(), ip.clone());


    let config = request
        .app_data::<Config>()
        .map(|data| Pin::new(data).get_ref().clone())
        .unwrap_or_else(Default::default);

    let token = credentials.token();

    let wrapper = RB.new_wrapper()
        .eq("aak_key", token)
        .and()
        .eq("aak_enabled", true);

    let result: Result<ApiKeys, _> = RB.fetch_by_wrapper(wrapper).await;

    if result.is_err()
    {
        let error = result.unwrap_err();
        log::info!("token not found!\n{}", error);
        return Err(AuthenticationError::from(config).into());
    }

    let key = result.unwrap();

    let blocked_wrapper = RB.new_wrapper()
        .eq("akb_key_id", key.aak_id)
        .and()
        .eq("akb_ip", ip);

    let blocked: Result<ApiKeysBlocked, _> = RB.fetch_by_wrapper(blocked_wrapper).await;

    if blocked.is_ok() {
        log::info!("IP blocked for this token!");
        return Err(AuthenticationError::from(config).into());
    }

    Ok(request)
}