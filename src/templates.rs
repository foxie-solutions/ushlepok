use axum_template::engine::Engine;
use handlebars::{Handlebars, TemplateError};

pub type AppEngine = Engine<Handlebars<'static>>;

pub fn init_handlebars() -> Result<Handlebars<'static>, TemplateError> {
    let mut hbs = Handlebars::new();
    hbs.register_template_string("layout", include_str!("../web/templates/layout.hbs"))?;
    hbs.register_template_string("index", include_str!("../web/templates/index.hbs"))?;
    hbs.register_template_string("add", include_str!("../web/templates/add.hbs"))?;
    hbs.register_template_string("rules", include_str!("../web/templates/rules.hbs"))?;
    Ok(hbs)
}
