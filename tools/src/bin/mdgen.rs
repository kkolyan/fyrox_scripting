use gen_common::code_model::Module;
use gen_common::writelnu;
use itertools::Itertools;
use md_gen_lib::md::cs_to_domain::{generate_cs_defined_domain, CSharpDomain};
use md_gen_lib::Naming;
use std::fs;

fn main() {
    generate_script_reference(Naming::Cs, "www/fyrox_cs", generate_md_cs());
    generate_script_reference(Naming::Lua, "www/fyrox_lua", generate_md_lua());
}

fn generate_script_reference(naming: Naming, book_dir: &str, code_base: Module) {
    let target_dir = format!("{book_dir}/src/scripting_api");
    let _ = fs::remove_dir_all(&target_dir);
    code_base.write_md(&target_dir);

    let mut breadcrumb = vec!["scripting_api".to_string()];
    let mut summary_section = String::new();
    generate_summary_section(
        "",
        &mut breadcrumb,
        &code_base.children.values().collect_vec(),
        &mut summary_section,
    );

    let mut summary_template = fs::read_to_string(format!("{book_dir}/src/SUMMARY.template.md")).unwrap();
    let summary = summary_template.replace("%GENERATED_SECTION%", &summary_section);
    fs::write(format!("{book_dir}/src/SUMMARY.md"), summary).unwrap();
}

fn generate_md_cs() -> Module {
    let domain = tools::get_fyrox_lite_domain();
    let cs_domain = generate_cs_defined_domain();
    md_gen_lib::generate_md::generate_md(&domain, &cs_domain, Naming::Cs)
}

fn generate_md_lua() -> Module {
    let domain = tools::get_combined_domain();
    let cs_domain = CSharpDomain { packages: vec![] };
    md_gen_lib::generate_md::generate_md(&domain, &cs_domain, Naming::Lua)
}

fn generate_summary_section(
    indent: &str,
    breadcrumb: &mut Vec<String>,
    mods: &[&Module],
    s: &mut String,
) {
    for m in mods.iter() {
        writelnu!(
            s,
            "{}{}- [{}]({}{}.md)",
            indent,
            "  ".repeat(breadcrumb.len()),
            &m.name,
            breadcrumb.iter().map(|it| format!("{it}/")).join(""),
            &m.name,
        );

        breadcrumb.push(m.name.clone());
        generate_summary_section(
            indent,
            breadcrumb,
            m.children.values().collect_vec().as_slice(),
            s,
        );
        breadcrumb.pop();
    }
}
