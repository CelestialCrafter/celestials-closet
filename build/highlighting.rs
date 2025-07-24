use std::{fmt::Write, sync::LazyLock};

use eyre::Result;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Tag, TagEnd};
use pulldown_cmark_escape::escape_html;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

const HIGHLIGHT_NAMES: &[&str] = &[
    "variable",
    "variable.builtin",
    "variable.member",
    "variable.parameter",
    "constant",
    "constant.builtin",
    "string",
    "string.escape",
    "string.regexp",
    "string.special",
    "boolean",
    "number",
    "type",
    "type.builtin",
    "attribute",
    "property",
    "function",
    "function.builtin",
    "constructor",
    "operator",
    "keyword",
    "punctuation",
    "comment",
    "markup.bold",
    "markup.italic",
    "markup.strikethrough",
    "markup.heading",
    "markup.link",
    "markup.raw",
    "tag",
];

macro_rules! highlight_config {
    ($(($global_name:ident, $lang_name:expr, $lang:expr, $hl_query:expr, $inject_query:expr, $local_query: expr)),*) => {
        $(static $global_name: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
            let mut config = HighlightConfiguration::new(
                $lang,
                $lang_name,
                $hl_query,
                $inject_query,
                $local_query,
            )
            .expect("should be able to register highlight configuration");

            config.configure(HIGHLIGHT_NAMES);

            config
        });)*

        fn get_config(language: &str) -> Option<&'static HighlightConfiguration> {
            match language {
                $($lang_name => Some(&$global_name),)*
                _ => None
            }
        }
    };
}

highlight_config!(
    (
        RUST_CONFIG,
        "rust",
        tree_sitter_rust::LANGUAGE.into(),
        tree_sitter_rust::HIGHLIGHTS_QUERY,
        tree_sitter_rust::INJECTIONS_QUERY,
        ""
    ),
    (
        JS_CONFIG,
        "javascript",
        tree_sitter_javascript::LANGUAGE.into(),
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        tree_sitter_javascript::INJECTIONS_QUERY,
        tree_sitter_javascript::LOCALS_QUERY
    ),
    (
        LUA_CONFIG,
        "lua",
        tree_sitter_lua::LANGUAGE.into(),
        tree_sitter_lua::HIGHLIGHTS_QUERY,
        tree_sitter_lua::INJECTIONS_QUERY,
        tree_sitter_lua::LOCALS_QUERY
    ),
    (
        FISH_CONFIG,
        "fish",
        tree_sitter_fish::language(),
        tree_sitter_fish::HIGHLIGHTS_QUERY,
        "",
        ""
    ),
    (
        NIX_CONFIG,
        "nix",
        tree_sitter_nix::LANGUAGE.into(),
        tree_sitter_nix::HIGHLIGHTS_QUERY,
        tree_sitter_nix::INJECTIONS_QUERY,
        ""
    ),
    (
        HTML_CONFIG,
        "html",
        tree_sitter_html::LANGUAGE.into(),
        tree_sitter_html::HIGHLIGHTS_QUERY,
        tree_sitter_html::INJECTIONS_QUERY,
        ""
    )
);

fn highlight_code<'a>(
    highlighter: &mut Highlighter,
    config: &HighlightConfiguration,
    code: &str,
) -> Result<String> {
    let highlights = highlighter.highlight(config, code.as_bytes(), None, |_| None)?;

    let mut html = String::with_capacity(code.len());

    for event in highlights.map(|e| {
        e.unwrap_or_else(|err| {
            println!("cargo:warning=highlight error: {}", err);
            HighlightEvent::HighlightEnd
        })
    }) {
        match event {
            HighlightEvent::Source { start, end } => escape_html(&mut html, &code[start..end]),
            HighlightEvent::HighlightStart(highlight) => {
                write!(html, "<span class=\"{}\">", HIGHLIGHT_NAMES[highlight.0])
            }
            HighlightEvent::HighlightEnd => write!(html, "</span>"),
        }
        .unwrap();
    }

    Ok(html)
}

pub fn inject_highlights<'a>(
    highlighter: &'a mut Highlighter,
    iter: impl Iterator<Item = Event<'a>>,
) -> impl Iterator<Item = Event<'a>> {
    iter.scan(None, move |state, event| {
        let inside = state.is_some();
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref language))) => {
                *state = get_config(language)
            }
            Event::Text(ref code) if inside => {
                match highlight_code(highlighter, state.unwrap(), code) {
                    Ok(highlighted) => return Some(Some(Event::Html(CowStr::from(highlighted)))),
                    Err(err) => println!("cargo:warning=could not highlight {}: {}", code, err),
                }
            }
            Event::End(TagEnd::CodeBlock) if inside => *state = None,
            _ => (),
        }

        Some(Some(event))
    })
    .filter_map(|e| e)
}
