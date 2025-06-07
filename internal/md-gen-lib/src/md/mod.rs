pub mod engine_class;
pub mod struct_class;
pub mod type_to_md;
pub mod enum_class;
pub mod csharp_metamodel;
pub mod cs_to_domain;

// #[macro_export]
// macro_rules! writelnu {
//     ($dst:expr, $($arg:tt)*) => {{
// 		use std::fmt::Write;
// 		writeln!($dst, $($arg)*).unwrap()
//     }}
// }