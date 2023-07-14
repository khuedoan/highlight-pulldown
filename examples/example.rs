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

use highlight_pulldown::highlight_with_theme;

fn main() {
    let markdown = r#"# Hello syntax highlighting
Here's some Python code:
```python
print("foo", 42)
```

And here's some Rust code:
```rust
enum Hello {
    World,
    SyntaxHighlighting,
}
```
"#;

    let events = pulldown_cmark::Parser::new(markdown);

    // The themes available are the same as in syntect:
    // - base16-ocean.dark,base16-eighties.dark,base16-mocha.dark,base16-ocean.light
    // - InspiredGitHub
    // - Solarized (dark) and Solarized (light)
    // The default theme is the same as in syntect: "base16-ocean.dark".
    // See also https://docs.rs/syntect/latest/syntect/highlighting/struct.ThemeSet.html#method.load_defaults .
    let events = highlight_with_theme(events, "base16-ocean.dark").unwrap();

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
