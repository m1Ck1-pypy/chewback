use utoipa_axum::router::OpenApiRouter;

use crate::database::AppState;

pub fn routes(state: &AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
}