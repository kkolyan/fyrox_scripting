use std::fs;
use md_gen_lib::md::cs_to_domain::generate_cs_defined_domain;
use md_gen_lib::Naming;

fn main() {
    let domain = tools::get_fyrox_lite_domain();
	let cs_domain = generate_cs_defined_domain();
	let target_dir = "generated-docs";
	let code_base = md_gen_lib::generate_md::generate_md(&domain, &cs_domain, Naming::Cs);
	let _ = fs::remove_dir_all(target_dir);
	code_base.write_md(target_dir);
}