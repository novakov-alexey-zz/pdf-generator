extern crate handlebars;
extern crate ini;
extern crate log;
extern crate serde;

use std::collections::HashMap;
use std::path::Path;

use handlebars_ext::I18Helper;

use super::handlebars_ext::*;

use self::handlebars::Handlebars;
use self::ini::Ini;
use self::ini::Error as IError;
use self::serde::ser::Serialize;

pub struct TemplateEngine {
    handlebars: Handlebars
}

#[derive(Debug)]
pub struct TemplateError(String);

impl TemplateEngine {
    pub fn new() -> Result<Self, TemplateError> {
        let handlebars = TemplateEngine::init_template_engine()?;
        Ok(TemplateEngine { handlebars })
    }

    fn init_template_engine() -> Result<Handlebars, TemplateError> {
        let mut handlebars = Handlebars::new();
        let path = Path::new("./templates");
        handlebars
            .register_templates_directory(".html", path)
            .map_err(|e| TemplateError(format!("Failed to register templates dir {}", e)))?;

        let i18_reg = TemplateEngine::load_i18_reg(&path)
            .map_err(|e| TemplateError(format!("Failed to load i18 file: {}", e)))?;

        TemplateEngine::register_helpers(&mut handlebars, i18_reg);

        let count = handlebars.get_templates().keys().len();
        info!("Number of registered templates: {:?}", count);

        if count > 0 {
            info!("Templates registered:");
            handlebars
                .get_templates()
                .keys()
                .for_each(|k| info!("{}", &k));
        }

        Ok(handlebars)
    }

    fn register_helpers(handlebars: &mut Handlebars, i18_reg: HashMap<String, String>) {
        handlebars.register_helper("array_length", Box::new(array_length_helper));
        handlebars.register_helper("i18", Box::new(I18Helper(i18_reg)));
        handlebars.register_helper("contains", Box::new(contains_helper));
    }

    fn load_i18_reg(path: &Path) -> Result<HashMap<String, String>, IError> {
        let conf = Ini::load_from_file(path.join(Path::new("i18.ini")))?;
        let mut reg = HashMap::new();
        for (_, prop) in conf.into_iter() {
            for (k, v) in prop.iter() {
                reg.insert(k.to_string(), v.to_string());
            }
        }
        info!("i18 registry is loaded, size {:?}", reg.len());
        Ok(reg)
    }

    pub fn render<T>(&self, template_name: &str, data: T)
                     -> Result<String, TemplateError> where T: Serialize + std::fmt::Debug {
        debug!("render template: {:?}", template_name);
        self.handlebars.render(&template_name, &data)
            .map_err(|e| TemplateError(e.to_string()))
    }
}