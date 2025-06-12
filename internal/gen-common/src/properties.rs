use std::ops::Deref;
use lite_model::{DataType, Method};
use crate::methods::{analyze_method_result, MethodResult};

pub struct Getter {
    pub instance: bool,
    pub prop_name: String,
    pub prop_type: DataType,
    pub description: String,
}
pub struct Setter {
    pub instance: bool,
    pub prop_name: String,
    pub prop_type: DataType,
    pub has_result: bool,
    pub description: String,
}

pub fn is_setter(method: &Method) -> bool {
    Setter::try_from(method).is_some()
}

pub fn is_getter(method: &Method) -> bool {
    Getter::try_from(method).is_some()
}

pub fn is_regular(method: &Method) -> bool {
    !is_setter(method) && !is_getter(method)
}

impl Setter {
    pub fn try_from(method: &Method) -> Option<Setter> {
        let MethodResult { may_fail, success_type: returns_value } = analyze_method_result(method);
        if matches!(returns_value, DataType::Unit) && method.signature.params.iter().filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub)).count() == 1 && method.method_name.starts_with("set_") {
            let prop_name = method.method_name.strip_prefix("set_").unwrap().to_string();
            let prop_type = method.signature.params.first().as_ref().unwrap().ty.clone();
            return Some(Setter {
                instance: method.instance,
                prop_name,
                prop_type,
                has_result: may_fail,
                description: method.description.clone(),
            });
        }
        None
    }
}

impl Getter {

    pub fn try_from(method: &Method) -> Option<Getter> {
        if method.signature.return_ty.is_some() && !method.signature.params.iter().any(|it| !matches!(&it.ty, DataType::UserScriptGenericStub)) && method.method_name.starts_with("get_") {
            let prop_name = method.method_name.strip_prefix("get_").unwrap().to_string();
            let prop_type = method.signature.return_ty.as_ref().unwrap().clone();
            return Some(Getter {
                instance: method.instance,
                prop_name,
                prop_type,
                description: method.description.clone(),
            });
        }
        None
    }
}