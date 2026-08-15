use std::{env, fmt::Write as _, fs, path::Path, path::PathBuf};

use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_transformer::{TransformOptions, Transformer};

fn main() {
    let scripts = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui/scripts");
    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join("scripts");

    println!("cargo::rerun-if-changed=ui/scripts");

    let mut manifest = String::from("pub const SCRIPTS: &[(&str, &str)] = &[\n");
    walk(&scripts, &scripts, &out, &mut manifest);
    manifest.push_str("];\n");

    let manifest_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join("scripts.rs");
    fs::write(&manifest_path, manifest).unwrap_or_else(|e| panic!("write {manifest_path:?}: {e}"));
}

fn walk(dir: &Path, root: &Path, out: &Path, manifest: &mut String) {
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("read {dir:?}: {e}")) {
        let path = entry.expect("read dir entry").path();
        if path.is_dir() {
            walk(&path, root, out, manifest);
            continue;
        }
        if !path.extension().is_some_and(|e| e == "ts") {
            continue;
        }

        let relative = path
            .strip_prefix(root)
            .expect("path under scripts root")
            .with_extension("js");
        let dst = out.join(&relative);
        fs::create_dir_all(dst.parent().expect("out parent")).expect("create out dir");
        transpile(&path, &dst);

        writeln!(
            manifest,
            "    (\"/{}\", include_str!(r\"{}\")),",
            relative.to_str().expect("utf-8 script path").replace('\\', "/"),
            dst.display()
        )
        .expect("write manifest entry");
    }
}

fn transpile(src: &Path, dst: &Path) {
    let source_text = fs::read_to_string(src).unwrap_or_else(|e| panic!("read {src:?}: {e}"));
    let source_type = SourceType::from_path(src).expect("source type");

    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, &source_text, source_type).parse();
    if !parsed.diagnostics.is_empty() {
        for error in &parsed.diagnostics {
            println!("cargo::error={}: {}", src.display(), error);
        }
        panic!("failed to parse {}", src.display());
    }

    let mut program = parsed.program;
    let scoping = SemanticBuilder::new().build(&program).semantic.into_scoping();
    let transformed = Transformer::new(&allocator, src, &TransformOptions::default())
        .build_with_scoping(scoping, &mut program);
    if !transformed.diagnostics.is_empty() {
        for error in &transformed.diagnostics {
            println!("cargo::error={}: {}", src.display(), error);
        }
        panic!("failed to transform {}", src.display());
    }

    let code = Codegen::new().build(&program).code;
    fs::write(dst, code).unwrap_or_else(|e| panic!("write {dst:?}: {e}"));
}