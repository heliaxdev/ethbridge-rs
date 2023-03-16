use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CargoTomlDepMeta {
    pub version: String,
    pub optional: bool,
}

const CARGO_TOML: &str = r#"
[package]
name = "{{ crate_name }}"
version = "{{ crate_version }}"
edition = "2021"

[features]
{% for feat, deps in features %}
{{ feat }} = {{ vecstr_quote(vec=deps) }}
{% endfor %}

[dependencies]
{% for dep, meta in dependencies %}
{% if dep == "ethbridge-structs" %}
{{ dep }} = { path = "../{{ dep }}", features = ["{{ feature_gate_ethers }}"] }
{% elif str_has_prefix(input_str=dep, prefix="ethbridge-") %}
{{ dep }} = { path = "../{{ dep }}" }
{% else %}
{{ dep }} = { version = "{{ meta.version }}", optional = {{ meta.optional }} }
{% endif %}
{% endfor %}
"#;

pub fn cargo() -> tera::Tera {
    let mut tera = tera::Tera::new("/dev/null/*").unwrap();
    tera.add_raw_template("Cargo.toml", CARGO_TOML)
        .expect("must compile template");
    tera.register_function("str_has_prefix", str_has_prefix);
    tera.register_function("vecstr_quote", vecstr_quote);
    tera
}

fn str_has_prefix(params: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let Some(tera::Value::String(input_str)) = params.get("input_str") else {
        return Err("no intput string found".into());
    };
    let Some(tera::Value::String(prefix)) = params.get("prefix") else {
        return Err("no prefix string found".into());
    };
    Ok(tera::Value::Bool(input_str.starts_with(prefix)))
}

fn vecstr_quote(params: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let Some(tera::Value::Array(array)) = params.get("vec") else {
        return Err("no intput vec of strings found".into());
    };
    Ok(tera::Value::Array(
        array
            .iter()
            .flat_map(|value| {
                let tera::Value::String(s) = value else {
                    return None;
                };
                Some(tera::Value::String(format!("\"{s}\"")))
            })
            .collect(),
    ))
}
