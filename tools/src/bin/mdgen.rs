use std::fs;
use gen_common::templating::render;
use md_gen_lib::Naming;

fn main() {
    let domain = tools::get_fyrox_lite_domain();
	let target_dir = "generated-docs";
	let code_base = md_gen_lib::generate_md::generate_md(&domain, Naming::Cs);
	let _ = fs::remove_dir_all(target_dir);
	code_base.write_md(target_dir);
}