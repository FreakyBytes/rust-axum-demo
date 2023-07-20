use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Redirect;
use axum::routing::{get, post};
use axum::{extract::State, response::IntoResponse, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::{instrument, warn};

use super::errors::ErrorMessage;
use crate::context::AppState;
use crate::db::link_visit::LinkVisit;
use crate::db::links::Link;

type ApiResult<T> = Result<T, ErrorMessage>;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_link))
        .route("/:code", get(follow_link))
        .route("/:code/meta", get(get_link_meta))
}

#[derive(Debug, Deserialize)]
struct CreateLinkPayload {
    url: String,
    code: Option<String>,
}

#[instrument(skip(ctx))]
async fn create_link(
    State(ctx): State<AppState>,
    Json(payload): Json<CreateLinkPayload>,
) -> ApiResult<impl IntoResponse> {
    let link = Link::create(&ctx.pool, &payload.url, payload.code.as_ref())
        .await
        .map(Json)
        .map_err(|err| {
            warn!(err = ?err, "Something, something can't save link");
            ErrorMessage::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create link.")
        })?;

    metrics::increment_counter!(
        "links_created",
        "with_code" => match payload.code {
            Some(_) => "true",
            None => "false",
        }
    );

    Ok(link)
}

#[instrument(skip(ctx))]
async fn follow_link(State(ctx): State<AppState>, Path(code): Path<String>) -> ApiResult<impl IntoResponse> {
    let link = Link::find_by_code(&ctx.pool, &code)
        .await
        .map_err(|err| {
            warn!(err = ?err, "Error while fetching link by code!");
            ErrorMessage::new(StatusCode::INTERNAL_SERVER_ERROR, "DB error")
        })?
        .ok_or_else(|| ErrorMessage::new(StatusCode::NOT_FOUND, "Link not found."))?;

    // // spawn background task to mark visit
    // let link_id = link.link_id;
    // tokio::spawn(async move {
    //     LinkVisit::mark_visit(&ctx.pool, link_id).await.ok();
    // });

    LinkVisit::mark_visit(&ctx.pool, link.link_id).await.ok();
    metrics::increment_counter!("links_visited");

    Ok(Redirect::temporary(&link.url))
}

#[derive(Debug, Serialize)]
struct LinkMetaResponse {
    url: String,
    code: String,
    visits: u64,
}

impl From<Link> for LinkMetaResponse {
    fn from(value: Link) -> Self {
        Self {
            url: value.url,
            code: value.code,
            visits: 0,
        }
    }
}

#[instrument(skip(ctx))]
async fn get_link_meta(State(ctx): State<AppState>, Path(code): Path<String>) -> ApiResult<Json<LinkMetaResponse>> {
    let link = match Link::find_by_code(&ctx.pool, &code).await {
        Err(err) => {
            warn!(err = ?err, "Error while fetching link by code!");
            return Err(ErrorMessage::new(StatusCode::INTERNAL_SERVER_ERROR, "DB error"));
        }
        Ok(None) => return Err(ErrorMessage::new(StatusCode::NOT_FOUND, "Link not found.")),
        Ok(Some(link)) => link,
    };
    let visit_count = LinkVisit::count_for_link_id(&ctx.pool, link.link_id)
        .await
        .unwrap_or_default();

    let mut resp: LinkMetaResponse = link.into();
    resp.visits = visit_count as u64;

    Ok(Json(resp))
}
