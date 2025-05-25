use error::abort_with_backtrace;

#[extend::ext]
pub impl<T> Result<T, crate::LangSpecificError> {
    fn handle_scripting_error(self) -> Option<T> {
        match self {
            Ok(it) => Some(it),
            Err(err) => {
                abort_with_backtrace!("\nScripting Runtime Error:\n{}", err);
            }
        }
    }
}
