# Highlight Pulldown Code

A small library crate to apply syntax highlighting to markdown parsed with [pulldown-cmark](https://crates.io/crates/pulldown-cmark).

The implementation is based on the discussion at [pulldown-cmark#167](https://github.com/raphlinus/pulldown-cmark/issues/167).

## Usage

The crate exposes a single function, `highlight`.
It takes an iterator over pulldown-cmark events and returns a corresponding `Vec<pulldown_cmark::Event>` where
code blocks have been substituted by HTML blocks containing highlighted code.

```rust
let markdown = r#"
```python
print("foo", 42)
\`\`\`

And here's some Rust code:
```rust
enum Hello {
    World,
    SyntaxHighlighting,
}
\`\`\`
"#;

let events = pulldown_cmark::Parser::new(markdown);
let events = highlight_with_theme(events, "base16-ocean.dark").unwrap();
let mut html = String::new();
pulldown_cmark::html::push_html(&mut html, events.into_iter());
```

## Contributing

If you happen to use this package, any feedback is more than welcome.

Contributions in the form of issues or patches via the GitLab repo are even more appreciated.