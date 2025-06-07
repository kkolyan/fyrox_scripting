use std::fs;
use convert_case::{Case, Casing};
use to_vec::ToVec;
use gen_common::code_model::{HierarchicalCodeBase, Module};

pub fn write_cs(dir: &str, code: Module) {
    fs::create_dir_all(dir).unwrap();
    fs::remove_dir_all(dir).unwrap();

    let mut nss = vec![];
    let children = code.children.values().to_vec();
    collect_uses(&children, "FyroxLite", &mut nss);
    nss.sort();
    write_cs_mods(dir, "FyroxLite", &children, &nss);
}

fn collect_uses(mods: &[&Module], parent_ns: &str, nss: &mut Vec<String>) {
    for m in mods {
        // let ns = format!("{}.{}", parent_ns, m.name.to_case(Case::Pascal));
        let ns = format!("{}", parent_ns);
        collect_uses(m.children.values().to_vec().as_slice(), ns.as_str(), nss);
        if !nss.contains(&ns) {
            nss.push(ns);
        }
    }
}

fn write_cs_mods(dir: &str, ns: &str, children: &[&Module], nss: &Vec<String>)  {
    if children.is_empty() {
        return;
    }
    let _ = fs::create_dir_all(dir);

    for m in children.iter() {
        write_cs_mod(m, ns, dir, nss);
    }
}
fn write_cs_mod(m: &Module, ns: &str, parent_dir: &str, nss: &Vec<String>)  {
    let mod_name = m.name.to_case(Case::Pascal);
    let dir = format!("{}/{}", parent_dir, mod_name);

    write_cs_mods(&dir, ns, m.children.values().to_vec().as_slice(), nss);

    let file = format!("{}/{}.cs", parent_dir, m.name.to_case(Case::Pascal));

    if let Some(code) = &m.code {
        let code = code.lines()
            .map(|it| it.strip_prefix("            ").unwrap_or(it))
            .to_vec()
            .join("\n");

        let mut s = String::new();
        s += "// ReSharper disable InconsistentNaming\n";
        s += "// ReSharper disable RedundantUnsafeContext\n";
        s += "// ReSharper disable UnusedMember.Global\n";
        s += "// ReSharper disable RedundantUsingDirective\n";
        for ns in nss.iter() {
            s += format!("using {};\n", ns).as_str();
        }
        s += format!("using System.Drawing;\n").as_str();
        s += format!("using System.Runtime.CompilerServices;\n").as_str();
        s += format!("using System.Runtime.InteropServices;\n").as_str();
        s += format!("using System.Collections;\n").as_str();
        s += format!("namespace {};\n", ns).as_str();
        s += code.as_str();
        fs::write(&file, s).unwrap();
        gen_common::fmt::fmt_file(file);
    }
}