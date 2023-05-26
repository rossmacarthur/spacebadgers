use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use indoc::formatdoc;
use once_cell::sync::Lazy;
use regex::Regex;
use walkdir::WalkDir;

/// Main entry point for the build script.
fn main() {
    IconSetCompiler::new()
        .compile(
            "Feather Icons",
            "feather_icons",
            "feather",
            "../vendor/feather/icons",
            "../vendor/feather/LICENSE",
            // Feather icons use `currentColor` for strokes, which doesn't work in our case.
            // We embed the code as a base64 data URI, so we need to replace `currentColor`.
            Some(|svg: &str| svg.replace("currentColor", "#fff")),
        )
        .finalize();
}

// TODO: Split this off into a utility crate.
// This is a copy of the `minify_svg` function from the spacebadgers crate.
static REGEX_MATCH_NEWLINE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\r?\n").unwrap());
static REGEX_MATCH_COMMENTS: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?s)<!--.*?-->").unwrap());
static REGEX_MATCH_BETWEEN_TAGS: Lazy<Regex> = Lazy::new(|| Regex::new(r"(>)(\s+)(<)").unwrap());
static REGEX_MATCH_TAG_END: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\s+)(/?>)").unwrap());
static REGEX_MATCH_START_END_WHITESPACE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s+|\s+$").unwrap());

fn minify_svg(str: impl AsRef<str>) -> String {
    let str = str.as_ref();
    let str = REGEX_MATCH_START_END_WHITESPACE.replace_all(str, "");
    let str = REGEX_MATCH_NEWLINE.replace_all(&str, " ");
    let str = REGEX_MATCH_COMMENTS.replace_all(&str, "");
    let str = REGEX_MATCH_BETWEEN_TAGS.replace_all(&str, "$1$3");
    let str = REGEX_MATCH_TAG_END.replace_all(&str, "$2");
    str.trim().to_string()
}
// END TODO

/// A single icon entry.
/// Used for generating the icon hashmap.
struct Icon {
    name: String,
    svg: String,
}

impl Icon {
    /// Generate a phf map entry for this icon.
    fn line(&self) -> String {
        let cleaned_svg = minify_svg(&self.svg);
        format!(
            r###""{name}" => r##"{svg}"##"###,
            name = self.name,
            svg = cleaned_svg.trim()
        )
    }
}

/// Basic information about an icon set.
/// Used for generating module declarations and exports.
struct IconSet {
    module: String,
    export: String,
}

/// Icon set compiler.
struct IconSetCompiler {
    icon_sets: Vec<IconSet>,
}

impl IconSetCompiler {
    /// Create a new icon set compiler.
    fn new() -> Self {
        Self {
            icon_sets: Vec::new(),
        }
    }

    /// Compile an icon set to a Rust module.
    fn compile(
        mut self,
        name: impl AsRef<str>,
        module: impl AsRef<str>,
        prefix: impl AsRef<str>,
        icon_path: impl AsRef<Path>,
        license_path: impl AsRef<Path>,
        post_process: Option<impl Fn(&str) -> String>,
    ) -> Self {
        let prefix = prefix.as_ref();
        let module = module.as_ref();
        let export = module.to_uppercase().replace([' ', '.'], "_");
        let mut icons = Vec::new();

        // Read and format the license
        let license = read_to_string(&license_path)
            .expect(&format!(
                "Unable to read license file: {:?}",
                license_path.as_ref()
            ))
            .split("\n")
            .map(|line| format!("//! {line}"))
            .collect::<Vec<_>>()
            .join("\n");

        // Find all SVG files
        for entry in WalkDir::new(icon_path).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && path.extension().map(|e| e == "svg").unwrap_or(false) {
                println!("cargo:rerun-if-changed={}", path.display());
                let icon_name = path
                    .file_stem()
                    .expect(&format!("Unable to get file stem for file: {:?}", path))
                    .to_string_lossy();
                let icon_name = format!("{prefix}-{icon_name}");
                let icon_svg =
                    read_to_string(path).expect(&format!("Unable to read file: {:?}", path));
                let icon_svg = post_process
                    .as_ref()
                    .map(|f| f(&icon_svg))
                    .unwrap_or(icon_svg);
                icons.push(Icon {
                    name: icon_name,
                    svg: icon_svg,
                });
            }
        }

        // Generate hashmap entries
        let hashmap_lines = icons
            .into_iter()
            .map(|icon| format!("        {line}", line = icon.line()))
            .collect::<Vec<_>>()
            .join(",\n");

        // Generate code
        let code = formatdoc! {r###"
            //! THIS FILE IS AUTO-GENERATED BY `build.rs`.
            //! DO NOT EDIT THIS FILE DIRECTLY.
            //!
            //! ## License
            //! ```plain,no_run
            {license}
            //! ```

            use phf::phf_map;

            use super::IconSet;

            pub const {export}: IconSet = IconSet {{
                name: "{name}",
                icons: phf_map! {{
            {hashmap_lines}
                }},
            }};
            "###,
            name = name.as_ref(),
        };

        // Write to file
        File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("src/icons/{module}.rs"))
            .expect(&format!(
                "Unable to open/create file: src/icons/{module}.rs"
            ))
            .write_all(code.trim().as_bytes())
            .expect(&format!("Unable to write to file: src/icons/{module}.rs"));

        // Register for finalization
        self.icon_sets.push(IconSet {
            module: module.to_string(),
            export,
        });

        self
    }

    fn finalize(self) {
        // Generate module declarations
        let modules = self
            .icon_sets
            .iter()
            .map(|set| format!("#[rustfmt::skip]\npub mod {};", set.module))
            .collect::<Vec<_>>()
            .join("\n");

        // Generate reexports
        let reexports = self
            .icon_sets
            .iter()
            .map(|set| format!("#[rustfmt::skip]\npub use {}::{};", set.module, set.export))
            .collect::<Vec<_>>()
            .join("\n");

        // Generate list of all icon sets
        let all_icon_sets = self
            .icon_sets
            .iter()
            .map(|set| format!("&{}", set.export))
            .collect::<Vec<_>>()
            .join(", ");

        // Generate icons.rs
        let code = formatdoc! {r###"
            //! THIS FILE IS AUTO-GENERATED BY `build.rs`.
            //! DO NOT EDIT THIS FILE DIRECTLY.

            pub mod icon_set;
            {modules}

            pub use icon_set::IconSet;
            {reexports}

            /// All available icon sets.
            pub const ALL_ICON_SETS: &[&IconSet] = &[{all_icon_sets}];

            /// Get the code for a named icon.
            pub fn get_icon_svg(name: impl AsRef<str>) -> Option<&'static str> {{
                let name = name.as_ref();
                ALL_ICON_SETS
                    .iter()
                    .find_map(|icon_set| icon_set.get(name))
            }}
        "###};

        // Write to file
        File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("src/icons.rs"))
            .expect("Unable to open/create file: src/icons.rs")
            .write_all(code.as_bytes())
            .expect("Unable to write to file: src/icons.rs");
    }
}
