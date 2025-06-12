pub mod by_package;
pub mod context;
pub mod templating;
pub mod code_model;
pub mod fmt;
pub mod properties;
pub mod methods;
pub mod doc;

#[macro_export]
macro_rules! writelnu {
    ($dst:expr, $($arg:tt)*) => {{
		use std::fmt::Write;
		writeln!($dst, $($arg)*).unwrap()
    }}
}
