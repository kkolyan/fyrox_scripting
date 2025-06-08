use gen_common::code_model::Module;
use gen_common::writelnu;
use itertools::Itertools;
use md_gen_lib::md::cs_to_domain::{generate_cs_defined_domain, CSharpDomain};
use md_gen_lib::Naming;
use std::fs;

fn main() {
    let mut summary = fs::read_to_string("www/src/SUMMARY.template.md").unwrap();
    generate_script_reference(Naming::Cs, "%GENERATED_SECTION_CS%", &mut summary);
    generate_script_reference(Naming::Lua, "%GENERATED_SECTION_LUA%", &mut summary);
    fs::write("www/src/SUMMARY.md", summary).unwrap();
}

fn generate_script_reference(naming: Naming, summary_placeholder: &str, summary: &mut String) {
    let code_base = match naming {
        Naming::Cs => generate_md_cs(),
        Naming::Lua => generate_md_lua(),
    };

    let target_dir = format!("www/src/{}", naming.md_root());
    let _ = fs::remove_dir_all(&target_dir);
    code_base.write_md(&target_dir);

    let mut breadcrumb = vec![naming.md_root().to_string()];
    let mut summary_section = String::new();
    generate_summary_section(
        "",
        &mut breadcrumb,
        &code_base.children.values().collect_vec(),
        &mut summary_section,
    );

    *summary = summary.replace(summary_placeholder, &summary_section);
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
