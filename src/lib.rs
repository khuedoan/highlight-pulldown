// Copyright (C) 2023 Enrico Guiraud
//
// This file is part of highlight-pulldown.
//
// highlight-pulldown is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// highlight-pulldown is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with highlight-pulldown. If not, see <http://www.gnu.org/licenses/>.

//! # Highlight Pulldown Code
//!
//! A small library crate to apply syntax highlighting to markdown parsed with [pulldown-cmark](https://crates.io/crates/pulldown-cmark).
//!
//! The implementation is based on the discussion at [pulldown-cmark#167](https://github.com/raphlinus/pulldown-cmark/issues/167).
//!
//! ## Usage
//!
//! The crate exposes a single function, `highlight`.
//! It takes an iterator over pulldown-cmark events and returns a corresponding `Vec<pulldown_cmark::Event>` where
//! code blocks have been substituted by HTML blocks containing highlighted code.
//!
//! ```rust
//! use highlight_pulldown::highlight_with_theme;
//!
//! let markdown = r#"
//! ```rust
//! enum Hello {
//!     World,
//!     SyntaxHighlighting,
//! }
//! ```"#;
//! let events = pulldown_cmark::Parser::new(markdown);
//!
//! // apply a syntax highlighting pass to the pulldown_cmark events
//! let events = highlight_with_theme(events, "base16-ocean.dark").unwrap();
//!
//! // emit HTML or further process the events as usual
//! let mut html = String::new();
//! pulldown_cmark::html::push_html(&mut html, events.into_iter());
//! ```
//!
//! ## Contributing
//!
//! If you happen to use this package, any feedback is more than welcome.
//!
//! Contributions in the form of issues or patches via the GitLab repo are even more appreciated.

use pulldown_cmark::{CodeBlockKind, CowStr, Event, Tag};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("theme '{0}' is not avaiable")]
    InvalidTheme(String),
    #[error("error highlighting code")]
    HighlightError(#[from] syntect::Error),
}

/// Apply syntax highlighting to pulldown-cmark events using the specified theme.
///
/// Take an iterator over pulldown-cmark's events, and (on success) return a new iterator
/// where code blocks have been turned into HTML text blocks with syntax highlighting.
///
/// Based on <https://github.com/raphlinus/pulldown-cmark/issues/167#issuecomment-448491422>.
pub fn highlight_with_theme<'a, It>(events: It, theme: &str) -> Result<Vec<Event<'a>>, Error>
where
    It: Iterator<Item = Event<'a>>,
{
    let mut in_code_block = false;

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let mut syntax = ss.find_syntax_plain_text();

    let theme = ts
        .themes
        .get(theme)
        .ok_or(Error::InvalidTheme(theme.to_string()))?;

    let mut to_highlight = String::new();
    let mut out_events = Vec::new();

    for event in events {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                match kind {
                    CodeBlockKind::Fenced(lang) => {
                        syntax = ss.find_syntax_by_token(&lang).unwrap_or(syntax)
                    }
                    CodeBlockKind::Indented => {}
                }
                in_code_block = true;
            }
            Event::End(Tag::CodeBlock(_)) => {
                if !in_code_block {
                    panic!("this should never happen");
                }
                let html = highlighted_html_for_string(&to_highlight, &ss, syntax, theme)?;

                to_highlight.clear();
                in_code_block = false;
                out_events.push(Event::Html(CowStr::from(html)));
            }
            Event::Text(t) => {
                if in_code_block {
                    to_highlight.push_str(&t);
                } else {
                    out_events.push(Event::Text(t));
                }
            }
            e => {
                out_events.push(e);
            }
        }
    }

    Ok(out_events)
}

/// Apply syntax highlighting to pulldown-cmark events using the default theme.
///
/// Take an iterator over pulldown-cmark's events, and (on success) return a new iterator
/// where code blocks have been turned into HTML text blocks with syntax highlighting.
///
/// Based on <https://github.com/raphlinus/pulldown-cmark/issues/167#issuecomment-448491422>.
pub fn highlight<'a, It>(events: It) -> Result<Vec<Event<'a>>, Error>
where
    It: Iterator<Item = Event<'a>>,
{
    highlight_with_theme(events, "base16-ocean.dark")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_theme() {
        let markdown = r#"
      ```python
      print("foo", 42)
      ```
   "#;

        let events = pulldown_cmark::Parser::new(markdown);

        // The themes available are the same as in syntect:
        // - base16-ocean.dark,base16-eighties.dark,base16-mocha.dark,base16-ocean.light
        // - InspiredGitHub
        // - Solarized (dark) and Solarized (light)
        // See also https://docs.rs/syntect/latest/syntect/highlighting/struct.ThemeSet.html#method.load_defaults .
        let events = highlight(events).unwrap();

        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, events.into_iter());

        let expected = r#"<pre style="background-color:#2b303b;">
<span style="color:#c0c5ce;">  ```python
</span><span style="color:#c0c5ce;">  print(&quot;foo&quot;, 42)
</span><span style="color:#c0c5ce;">  ```
</span></pre>
"#;
        assert_eq!(html, expected);
    }

    #[test]
    fn with_theme() {
        let markdown = r#"```python
print("foo", 42)
```
"#;

        let events = pulldown_cmark::Parser::new(markdown);
        let events = highlight_with_theme(events, "Solarized (dark)").unwrap();
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, events.into_iter());

        let expected = r#"<pre style="background-color:#002b36;">
<span style="color:#859900;">print</span><span style="color:#657b83;">(</span><span style="color:#839496;">&quot;</span><span style="color:#2aa198;">foo</span><span style="color:#839496;">&quot;, </span><span style="color:#6c71c4;">42</span><span style="color:#657b83;">)
</span></pre>
"#;

        assert_eq!(html, expected);
    }
}
