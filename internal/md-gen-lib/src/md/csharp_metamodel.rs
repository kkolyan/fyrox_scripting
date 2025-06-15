use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsFile {
    #[serde(default)]
    pub classes: Vec<CsClass>,
    #[serde(default)]
    pub enums: Vec<CsEnum>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsEnum {
    pub name: String,
    pub ns: String,
    #[serde(default)]
    pub members: Vec<CsEnumMember>,
    #[serde(default)]
    pub description: Vec<CsXmlNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsEnumMember {
    pub name: String,
    #[serde(default)]
    pub description: Vec<CsXmlNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsClass {
    pub name: String,
    pub ns: String,
    pub is_struct: bool,
    #[serde(default)]
    pub methods: Vec<CsMethod>,
    #[serde(default)]
    pub operators: Vec<CsMethod>,
    #[serde(default)]
    pub constructors: Vec<CsConstructor>,
    #[serde(default)]
    pub fields: Vec<CsField>,
    #[serde(default)]
    pub properties: Vec<CsProperty>,
    #[serde(default)]
    pub description: Vec<CsXmlNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsMethod {
    pub name: String,
    pub is_static: bool,
    pub return_ty: CsType,
    #[serde(default)]
    pub parameters: Vec<CsParam>,
    #[serde(default)]
    pub description: Vec<CsXmlNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsConstructor {
    #[serde(default)]
    pub parameters: Vec<CsParam>,
    #[serde(default)]
    pub description: Vec<CsXmlNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsField {
    pub name: String,
    pub is_static: bool,
    pub is_const: bool,
    pub ty: CsType,
    #[serde(default)]
    pub description: Vec<CsXmlNode>,
    pub initializer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsParam {
    pub name: String,
    pub ty: CsType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsProperty {
    pub name: String,
    pub is_static: bool,
    pub ty: CsType,
    pub get: bool,
    pub set: bool,
    pub expression: Option<String>,
    #[serde(default)]
    pub description: Vec<CsXmlNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsType {
    pub name: String,
    #[serde(default)]
    pub args: Vec<CsType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsXmlNode {
    pub text: Option<String>,
    pub element: Option<CsXmlElement>,
    pub unknown: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsXmlElement {
    pub name: String,
    #[serde(default)]
    pub children: Vec<CsXmlNode>,
    #[serde(default)]
    pub attrs: HashMap<String, String>,
}
