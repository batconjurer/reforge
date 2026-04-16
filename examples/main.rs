use std::collections::HashMap;
use std::sync::Arc;

use reforge::{MacroRules, PreprocessingData};
use solar::sema::Gcx;
use solar::sema::hir::ContractKind;

fn main() -> eyre::Result<()> {
    let mut macros = MacroRules::default();
    macros.rules.push(do_nothing);
    macros.rules.push(print_name);
    macros.run()
}

fn do_nothing(_: &Gcx, _: &mut PreprocessingData<'_>) -> foundry_compilers::error::Result<()> {
    Ok(())
}

/// A macro that adds a function for each struct definition that prints the struct name.
/// The function is injected into the pre-existing `Library{name}` library.
fn print_name(ctx: &Gcx, data: &mut PreprocessingData<'_>) -> foundry_compilers::error::Result<()> {
    // Collect (offset, injected_text) per file path.
    let mut insertions: HashMap<std::path::PathBuf, Vec<(usize, String)>> = HashMap::new();

    for struct_def in ctx.hir.structs() {
        let Some(source) = ctx.sources.get(struct_def.source) else {
            continue;
        };
        let Some(path) = source.file.name.as_real() else {
            continue;
        };

        if !data.input.contains_key(path) {
            continue;
        }

        let name = struct_def.name.name;
        let library_name = format!("Library{name}");

        // Find the pre-existing library named `Library{name}` in the same source file.
        let Some(library) = ctx.hir.contracts().find(|c| {
            c.source == struct_def.source
                && c.kind == ContractKind::Library
                && c.name.name.as_str() == library_name
        }) else {
            println!("No library named {library_name} found, skipping struct {name}");
            continue;
        };

        // Insert just before the closing `}` of the library body.
        let close_brace_offset = (library.span.hi().0 - source.file.start_pos.0) as usize - 1;
        let func = format!(
            "\n    function print_{name}() public pure returns (string memory) {{ return \"{name}\"; }}\n"
        );
        println!("Injecting function for struct {name} into {library_name}");
        insertions
            .entry(path.to_path_buf())
            .or_default()
            .push((close_brace_offset, func));
    }

    for (path, mut inserts) in insertions {
        let src = data.input.get_mut(&path).unwrap();
        // Apply in reverse offset order so earlier positions aren't shifted.
        inserts.sort_by(|a, b| b.0.cmp(&a.0));
        let content = Arc::make_mut(&mut src.content);
        for (offset, text) in inserts {
            content.insert_str(offset, &text);
        }
    }
    Ok(())
}
