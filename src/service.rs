extern crate dotenv;
extern crate serde;
extern crate uuid;
extern crate log;

use self::serde::ser::Serialize;
use self::uuid::Uuid;
use service::dotenv::dotenv;
use std::env;
use std::io::Write;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;
use super::templates::TemplateEngine;

const USE_STDIN_MARKER: &str = "-";
const WKHTMLTOPDF_CMD: &str = "wkhtmltopdf";

type PdfPath = String;

pub struct ReportService {
    template_engine: TemplateEngine,
    work_dir: String,
}

#[derive(Debug)]
pub struct ServiceError(String);

#[derive(Debug)]
pub struct RenderingError(String);

impl ReportService {
    pub fn new() -> Result<Self, ServiceError> {
        dotenv().ok();

        let work_dir = env::var("WORK_DIR").unwrap_or_else(|_| "target/work_dir".to_string());
        let template_engine = TemplateEngine::new()
            .map_err(|e| ServiceError(
                format!("Failed to create template engine, error: {:?}", e)
            ))?;

        Ok(ReportService { template_engine, work_dir })
    }

    pub fn render<T>(&self, template_name: String, data: T)
                     -> Result<PdfPath, RenderingError> where T: Serialize + std::fmt::Debug {
        debug!("rendering report for data {:?}", &data);
        let html = self.template_engine.render(&template_name, &data)
            .map_err(|e| RenderingError(format!("Failed to render, error: {:?}", e)))?;

        let destination_pdf = self.dest_name(&template_name);

        debug!("destination PDF {}", &destination_pdf);
        let output = ReportService::run_blocking(html, &destination_pdf)?;

        debug!("status: {}", output.status);
        debug!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        debug!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        if output.status.success() {
            Ok(destination_pdf)
        } else {
            Err(RenderingError(format!("Failed to render template: {:?}", template_name)))
        }
    }

    fn dest_name(&self, template_name: &str) -> PdfPath {
        format!("{}/{}-{}.pdf", self.work_dir, Uuid::new_v4(), template_name)
    }

    fn run_blocking(html: String, destination_pdf: &str) -> Result<Output, RenderingError> {
        let mut child = Command::new(WKHTMLTOPDF_CMD)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .arg(USE_STDIN_MARKER)
            .arg(&destination_pdf)
            .spawn()
            .map_err(|e| RenderingError(format!("Failed to spawn child process: {}", e)))?;
        {
            let stdin = child.stdin.as_mut()
                .ok_or(RenderingError("Failed to open stdin".to_string()))?;

            stdin.write_all(html.as_bytes())
                .map_err(|e| RenderingError(format!("Failed to write HTML into stdin, error: {}", e)))?;
        }

        let output = child.wait_with_output()
            .map_err(|e| RenderingError(format!("Failed to read stdout, error: {}", e)))?;

        Ok(output)
    }
}