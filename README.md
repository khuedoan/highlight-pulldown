[![pipeline status](https://gitlab.com/eguiraud/highlight-pulldown/badges/main/pipeline.svg)](https://gitlab.com/eguiraud/highlight-pulldown/-/commits/main)
[![crates.io](https://img.shields.io/crates/v/highlight-pulldown.svg)](https://crates.io/crates/highlight-pulldown)

# Highlight Pulldown Code

A small library crate to apply syntax highlighting to markdown parsed with [pulldown-cmark](https://crates.io/crates/pulldown-cmark).

The implementation is based on the discussion at [pulldown-cmark#167](https://github.com/raphlinus/pulldown-cmark/issues/167).

## Usage

The crate exposes a single function, `highlight`.
It takes an iterator over pulldown-cmark events and returns a corresponding `Vec<pulldown_cmark::Event>` where
code blocks have been substituted by HTML blocks containing highlighted code.

```rust
let markdown = r#"
```rust
enum Hello {
    World,
    SyntaxHighlighting,
}
\`\`\`
"#;
let events = pulldown_cmark::Parser::new(markdown);

// apply a syntax highlighting pass to the pulldown_cmark events
let events = highlight_with_theme(events, "base16-ocean.dark").unwrap();

// emit HTML or further process the events as usual
let mut html = String::new();
pulldown_cmark::html::push_html(&mut html, events.into_iter());
```

## Contributing

If you happen to use this package, any feedback is more than welcome.

Contributions in the form of issues or patches via the GitLab repo are even more appreciated.