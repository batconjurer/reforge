//! Utilities for printing macro-expanded Solidity sources to stdout.

use std::path::Path;

use foundry_compilers::artifacts::Sources;
use glob::MatchOptions;

use crate::Macro;
use crate::testing::expand_macros;

/// Expands `macro_rules` over the Solidity sources in `source`, then prints
/// the content of every file whose path (relative to `source`) matches `glob`
/// to stdout. Files are printed in alphabetical order, each preceded by a
/// header showing its relative path.
pub fn display_expanded(
    source: impl AsRef<Path>,
    glob: &str,
    macro_rules: &[Macro],
) -> eyre::Result<()> {
    let source = source.as_ref();
    let sources = expand_macros(source, None, macro_rules)?;
    display_sources(source, glob, &sources)
}

/// Filters `sources` by `glob` relative to `root` and prints each matching
/// file's content to stdout in alphabetical order, each preceded by a header
/// showing its relative path.
pub fn display_sources(root: &Path, glob: &str, sources: &Sources) -> eyre::Result<()> {
    let pattern = glob::Pattern::new(glob)?;

    let mut matched: Vec<_> = sources
        .iter()
        .filter_map(|(path, src)| {
            let relative = path.strip_prefix(root).ok()?;
            pattern
                .matches_path_with(relative, MatchOptions {
                    require_literal_separator: true,
                    ..Default::default()
                })
                .then_some((relative.to_path_buf(), src.content.clone()))
        })
        .collect();

    matched.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (relative, content) in &matched {
        println!("=== {} ===", relative.display());
        println!("{content}");
    }

    Ok(())
}