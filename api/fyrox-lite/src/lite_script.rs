use lite_macro::lite_api;

use crate::spi::UserScript;

#[derive(Debug, Clone)]
pub struct LiteGlobalScript;

#[lite_api(class=GlobalScript)]
impl LiteGlobalScript {
	/// find a global script by type
	pub fn get<T: UserScript>(class_id: T::ClassId, _stub: T::UserScriptGenericStub) -> Result<T, T::LangSpecificError> {
		T::find_global_script(&class_id)
	}
}