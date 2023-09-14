use deno_ast::MediaType;
// use deno_ast::{SourceRange, SourceTextInfo, SyntaxError};
use deno_lint::diagnostic::LintDiagnostic;
use deno_lint::linter::LinterBuilder;
use deno_lint::rules::{get_all_rules, get_recommended_rules};
use std::path::Path;
use wasm_bindgen::prelude::*;

// use serde::{Deserialize, Serialize};

// #[derive(Serialize)]
// pub struct LintResult {
//     result: Vec<LintDiagnostic>,
//     error: Diagnostic,
// }

#[wasm_bindgen(js_name = MyStruct)]
pub struct MyLintDiagnostic(Vec<LintDiagnostic>);

#[wasm_bindgen]
pub fn main() {
    let file_name = "file:///src.ts";
    let all_rules = false;
    let linter = LinterBuilder::default()
        .rules(if all_rules {
            get_all_rules()
        } else {
            get_recommended_rules()
        })
        .media_type(get_media_type(Path::new(file_name)))
        .ignore_diagnostic_directive("eslint-disable-next-line")
        .build();

    let source_string = "var a = 1;";

    let res = linter.lint(file_name.to_string(), source_string.to_owned());
    // match linter.lint(file_name.to_string(), source_string.to_owned()) {
    //     Ok((_, result)) => Ok("success".to_string()),
    //     Err(_err) => Err("Syntax Error".to_string()),
    // }
    match res {
        Ok((parsed, diag)) => {
            println!("{:?}", parsed);
            println!("");
            for d in diag {
                println!("{:?}", d);
            }
        }
        Err(error) => println!("{:?}", error),
    };
    println!("aaa");
    return;
    // .map_err(|e| {
    //     Error::new(
    //         Status::GenericFailure,
    //         format!("Lint failed: {e}, at: {file_name}"),
    //     )
    // })?;

    // return Ok(file_diagnostics);
}

#[inline(always)]
fn get_media_type(p: &Path) -> MediaType {
    match p.extension().and_then(|e| e.to_str()) {
        Some("tsx") => MediaType::Tsx,
        Some("jsx") => MediaType::Jsx,
        Some("js") | Some("mjs") => MediaType::JavaScript,
        Some("ts") => MediaType::TypeScript,
        _ => MediaType::Tsx,
    }
}
