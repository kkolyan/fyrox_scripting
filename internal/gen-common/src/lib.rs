pub mod by_package;
pub mod code_model;
pub mod context;
pub mod doc;
pub mod fmt;
pub mod methods;
pub mod properties;
pub mod templating;

#[macro_export]
macro_rules! writelnu {
    ($dst:expr, $($arg:tt)*) => {{
		use std::fmt::Write;
		writeln!($dst, $($arg)*).unwrap()
    }}
}
