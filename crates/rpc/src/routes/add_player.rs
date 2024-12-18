use rocket::{get, http::Status, serde::json::Json, State};
use serde::Serialize;

use crate::model::cluster::{shared::AddPlayerLocalResponse, ClusterState};

#[derive(Serialize)]
pub struct AddPlayerResponse {
    ip: String,
    player_state: String,
    admin_pkh: String,
}

#[get("/add_player?<address>&<id>")]
pub async fn add_player(
    address: &str,
    id: &str,
    state: &State<ClusterState>,
) -> Result<Json<AddPlayerResponse>, Status> {
    let node = state.get_node_by_id(id).ok_or(Status::NotFound)?;

    let (external_url, local_url): (String, String) = node
        .status
        .as_ref()
        .map(|status| {
            (
                status.external_url.clone(),
                status
                    .local_url
                    .clone()
                    .replace("ws://", "http://")
                    .replace("4001", "8000"),
            )
        })
        .unwrap_or_default();

    let url = local_url + "/game/add_player?address=" + address;
    let response = reqwest::get(url).await.map_err(|_| Status::BadGateway)?;

    let body = response
        .json::<AddPlayerLocalResponse>()
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(AddPlayerResponse {
        ip: external_url,
        player_state: body.player_state,
        admin_pkh: body.admin_pkh,
    }))
}
