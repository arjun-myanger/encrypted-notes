use crate::storage::Storage;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::{
    Filter, Rejection, Reply,
    reply::{json, with_status},
};

pub fn note_routes(
    storage: Arc<Storage>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let save = warp::path("create")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_storage(storage.clone()))
        .and_then(handle_save);

    let get = warp::path!("read" / String)
        .and(warp::get())
        .and(with_storage(storage))
        .and_then(handle_read);

    save.or(get)
}

async fn handle_save(note: String, storage: Arc<Storage>) -> Result<impl Reply, Rejection> {
    let id = storage.save_note(&note);
    Ok(json(
        &serde_json::json!({"url": format!("https://yourdomain.com/read/{}", id)}),
    ))
}

async fn handle_read(id: String, storage: Arc<Storage>) -> Result<Box<dyn Reply>, Rejection> {
    if let Some(note) = storage.get_and_delete(&id) {
        let success_response = json(&serde_json::json!({ "note": note }));
        Ok(Box::new(success_response))
    } else {
        let error_response =
            json(&serde_json::json!({ "error": "Note not found, It has already been read!!" }));
        Ok(Box::new(with_status(error_response, StatusCode::NOT_FOUND)))
    }
}

fn with_storage(
    storage: Arc<Storage>,
) -> impl Filter<Extract = (Arc<Storage>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || storage.clone())
}
