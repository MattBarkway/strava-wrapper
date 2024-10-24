use crate::activities::User;
use crate::query;
use crate::query::ErrorWrapper;

enum Format {
    GPX,
    TCX,
}

pub async fn export(token: &str, id: &str, format: Format) -> Result<Vec<User>, ErrorWrapper> {
    let format_str = match format {
        Format::GPX => "export_gpx",
        Format::TCX => "export_tcx",
    };
    query::get(&format!("routes/{}/{}", id, format_str), token).await
}
pub async fn get() {}
pub async fn list() {}
