use std::fs;
use std::path::Path;

fn main() {
    let problems_dir = Path::new("src/problems");
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let mut problems: Vec<(u32, String, bool)> = Vec::new();

    for entry in fs::read_dir(problems_dir).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name().to_string_lossy().to_string();
        if let Some(num_str) = file_name.strip_prefix('p').and_then(|s| s.strip_suffix(".rs")) {
            if let Ok(num) = num_str.parse::<u32>() {
                let contents = fs::read_to_string(entry.path()).unwrap();
                let has_input = contents.contains("fn solve_with_input");
                let mod_name = file_name.strip_suffix(".rs").unwrap().to_string();
                problems.push((num, mod_name, has_input));
            }
        }
    }

    problems.sort_by_key(|(num, _, _)| *num);

    let mut code = String::new();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // Module declarations with explicit paths back to src/problems/
    for (_, mod_name, _) in &problems {
        code += &format!(
            "#[path = \"{}/src/problems/{}.rs\"]\npub mod {};\n",
            manifest_dir, mod_name, mod_name
        );
    }

    // ALL_PROBLEMS
    code += "\npub const ALL_PROBLEMS: &[u32] = &[\n";
    for (num, _, _) in &problems {
        code += &format!("    {},\n", num);
    }
    code += "];\n";

    // solve_with_input — used by guest and host
    code += "\npub fn solve_with_input(problem: u32, input: &str) -> String {\n";
    code += "    match problem {\n";
    for (num, mod_name, has_input) in &problems {
        if *has_input {
            code += &format!("        {} => {}::solve_with_input(input),\n", num, mod_name);
        } else {
            code += &format!("        {} => {}::solve(),\n", num, mod_name);
        }
    }
    code += "        _ => panic!(\"Problem {} not implemented\", problem),\n";
    code += "    }\n}\n";

    // input_file — maps problem numbers to input file paths
    code += "\npub fn input_file(problem: u32) -> Option<&'static str> {\n";
    code += "    match problem {\n";
    for (num, _, has_input) in &problems {
        if *has_input {
            code += &format!("        {} => Some(\"inputs/p{:03}.txt\"),\n", num, num);
        }
    }
    code += "        _ => None,\n";
    code += "    }\n}\n";

    // solve — CLI version that loads input files (std-io only)
    code += "\n#[cfg(feature = \"std-io\")]\n";
    code += "pub fn solve(problem: u32) -> String {\n";
    code += "    let input = match input_file(problem) {\n";
    code += "        Some(path) => std::fs::read_to_string(path).unwrap(),\n";
    code += "        None => String::new(),\n";
    code += "    };\n";
    code += "    solve_with_input(problem, &input)\n";
    code += "}\n";

    fs::write(Path::new(&out_dir).join("problems_generated.rs"), code).unwrap();

    println!("cargo:rerun-if-changed=src/problems");
}
