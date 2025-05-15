#[cfg(all(feature = "v2", feature = "tokenization_v2"))]
use std::sync::Arc;

#[cfg(all(feature = "v2", feature = "tokenization_v2"))]
use actix_web::{web, HttpRequest, HttpResponse};
#[cfg(all(feature = "v2", feature = "tokenization_v2"))]
use api_models;
#[cfg(all(feature = "v2", feature = "tokenization_v2"))]
use common_utils::{
    ext_traits::{BytesExt, Encode},
    id_type,
};
#[cfg(all(feature = "v2", feature = "tokenization_v2"))]
use error_stack::ResultExt;
#[cfg(all(feature = "v2", feature = "tokenization_v2"))]
use hyperswitch_domain_models;
#[cfg(all(feature = "v2", feature = "tokenization_v2"))]
use masking::Secret;
#[cfg(all(feature = "v2", feature = "tokenization_v2"))]
use router_env::{instrument, logger, tracing, Flow};
#[cfg(all(feature = "v2", feature = "tokenization_v2"))]
use serde::Serialize;

#[cfg(all(feature = "v2", feature = "tokenization_v2"))]
use crate::{
    core::{
        api_locking,
        errors::{self, RouterResult},
        tokenization,
    },
    headers::X_CUSTOMER_ID,
    routes::{app::StorageInterface, AppState, SessionState},
    services::{self, api as api_service, authentication as auth},
    types::{api, domain, payment_methods as pm_types},
};

#[instrument(skip_all, fields(flow = ?Flow::TokenizationCreate))]
#[cfg(all(feature = "v2", feature = "tokenization_v2"))]
pub async fn create_token_vault_api(
    state: web::Data<AppState>,
    req: HttpRequest,
    json_payload: web::Json<api_models::tokenization::GenericTokenizationRequest>,
) -> HttpResponse {
    let flow = Flow::TokenizationCreate;
    let customer_id = payload.customer_id.clone();
    Box::pin(api_service::server_wrap(
        flow,
        state,
        &req,
        payload,
        |state, auth: auth::AuthenticationData, request, _| async move {
            tokenization::create_vault_token_core(
                state,
                &auth.merchant_account,
                &auth.key_store,
                request,
            )
            .await
        },
        auth::api_or_client_auth(
            &auth::V2ApiKeyAuth {
                is_connected_allowed: false,
                is_platform_allowed: false,
            },
            &auth::V2ClientAuth(common_utils::types::authentication::ResourceId::Customer(
                customer_id,
            )),
            req.headers(),
        ),
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

#[instrument(skip_all, fields(flow = ?Flow::TokenizationRetrieve))]
#[cfg(all(feature = "v2", feature = "tokenization_v2"))]
pub async fn get_token_vault_api(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<id_type::GlobalTokenId>,
    query: web::Query<api_models::tokenization::TokenizationQueryParameters>,
) -> HttpResponse {
    let reveal_flag = matches!(query.reveal, Some(true));
    let token_id = path.into_inner();
    Box::pin(api_service::server_wrap(
        Flow::TokenizationRetrieve,
        state,
        &req,
        token_id.clone(),
        |state, auth: auth::AuthenticationData, token_id, _| async move {
            tokenization::get_token_vault_core(
                state,
                &auth.merchant_account,
                &auth.key_store,
                (token_id, reveal_flag),
            )
            .await
        },
        &auth::V2ApiKeyAuth {
            is_connected_allowed: false,
            is_platform_allowed: false,
        },
        api_locking::LockAction::NotApplicable,
    ))
    .await
}
