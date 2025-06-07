use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CsClass {
    pub name: String,
    pub methods: Vec<CsMethod>,
    pub operators: Vec<CsMethod>,
    pub fields: Vec<CsParam>,
    pub properties: Vec<CsProperty>,
    pub description: Vec<CsXmlNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CsMethod {
    pub name: String,
    pub is_static: bool,
    pub return_ty: CsType,
    pub parameters: Vec<CsParam>,
    pub description: Vec<CsXmlNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CsParam {
    pub name: String,
    pub ty: CsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CsProperty {
    pub name: String,
    pub ty: CsType,
    pub get: bool,
    pub set: bool,
    pub description: Vec<CsXmlNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CsType {
    pub name: String,
    pub args: Vec<CsType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CsXmlNode {
    pub text: Option<String>,
    pub element_name: Option<String>,
    pub element_children: Option<Vec<CsXmlNode>>,
    pub element_attrs: Option<HashMap<String, String>>,
    pub unknown: Option<serde_json::Value>,
}
