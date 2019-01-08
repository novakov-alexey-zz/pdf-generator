extern crate log;
extern crate rocket;

use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use service::ReportService;

use self::rocket::State;
use rocket::Rocket;

#[derive(Deserialize)]
pub struct GetReport {
    template_name: String,
    user_params: JsonValue,
}

#[post("/generate", format = "application/json", data = "<req>")]
pub fn generate_report(service: State<ReportService>, req: Json<GetReport>) -> Result<NamedFile, Json<String>> {
    let params = req.0.user_params;
    let report = service.render(req.0.template_name, params);

    report
        .map_err(|e| Json(format!("Failed to generate report: {:?}", e)))
        .and_then(|path| NamedFile::open(path).map_err(|e| Json(e.to_string())))
}

pub fn mount_routes(service: ReportService) -> Rocket {
    rocket::ignite()
        .manage(service)
        .mount(
            "/api/v1",
            routes![generate_report],
        )
}