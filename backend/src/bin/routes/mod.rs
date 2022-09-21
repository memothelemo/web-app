pub mod api;

// react-router handling
#[allow(unused)]
#[rocket::get("/<_..>", rank = 2)]
pub async fn file_server_fallback() -> Option<rocket::fs::NamedFile> {
	use std::path::Path;
	use rocket::fs::NamedFile;

	NamedFile::open(Path::new("../frontend/build/index.html")).await.ok()
}
