//! Document fonts from the optional craft-fonts build input (https://github.com/storytold/craft-fonts).
//! With `CRAFT_FONTS_DIR=<checkout>` the families of [`DOCUMENT_FAMILIES`] are embedded as
//! `CRAFT_FONTS`; unset, `CRAFT_FONTS` is empty. Web (wasm32) builds embed only BIZ UDPGothic
//! Regular, to stay within the web size budget. With `CRAFT_FONTS_REQUIRED=1` (releases), a build
//! fails when the checkout lacks one of the families. It only reads the local checkout: no network.
//! The interface's own faces (the CJK interface cuts, emoji, other scripts) are embedded by
//! `wordcraft-ui-egui`'s `build.rs`.

/// The families documents fall back to when no installed font has a character: Japanese (Mincho
/// first, the serif default), Simplified Chinese for the ideographs the Japanese faces lack, Arabic
/// (#1652) and Thai text, the symbol faces Word's symbol fonts map to, colour emoji, and Noto Sans
/// Math for equations.
const DOCUMENT_FAMILIES: &[&str] = &[
    "Shippori Mincho",
    "BIZ UDMincho",
    "BIZ UDPGothic",
    "Noto Sans CJK SC",
    "Noto Sans Arabic",
    "Noto Sans Thai",
    "Noto Sans Symbols",
    "Noto Sans Symbols 2",
    "Noto Color Emoji",
    "Noto Sans Math",
];

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    let wasm = craft_fonts_build::target_is_wasm();
    let written = craft_fonts_build::Embed::documents(DOCUMENT_FAMILIES)
        .filter(move |face| !wasm || (face.family == "BIZ UDPGothic" && face.style == "Regular"))
        .write("craft_fonts.rs");
    if let Err(e) = written {
        println!("cargo::error={e}");
    }
}
