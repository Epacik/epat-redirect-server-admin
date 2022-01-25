use std::pin::Pin;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use rbatis::crud::CRUD;
use crate::database::RB;
use crate::database::ApiKeys;

pub async fn validate_authorization(request: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, actix_web::Error> {
    log::info!("authenticating");
    log::info!("{}",credentials.token());

    let config = request
        .app_data::<Config>()
        .map(|data| Pin::new(data).get_ref().clone())
        .unwrap_or_else(Default::default);

    let token = credentials.token();

    let wrapper = RB.new_wrapper()
        .eq("aak_key", token)
        .and()
        .eq("aak_enabled", true);

    let result :Result<ApiKeys, _> = RB.fetch_by_wrapper(wrapper).await;

    if result.is_err()
    {
        let error = result.unwrap_err();
        log::info!("token not found!\n{}", error);
        return Err(AuthenticationError::from(config).into());
    }
    Ok(request)
}