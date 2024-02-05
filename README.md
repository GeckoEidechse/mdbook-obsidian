# mdbook-obsidian

[mdBook](https://github.com/rust-lang/mdBook) preprocessor to render [Obsidian](https://obsidian.md/) specific syntax in mdBook.

To see the list of existing and supported syntax, see [this GitHub issue](https://github.com/GeckoEidechse/mdbook-obsidian/issues/1).

> ⚠️ WIP ⚠️
>
> This mdBook preprocessor is very much work-in-progress and currently only supports a small subset of the total Obsidian specific syntax.
> Contributions to expand the supported syntax are more than welcome <3
>
> If you'd like to take over maintainership or ownership of this crate, please get in touch via an issue in the GitHub repo of this crate.

## Usage

First, install the preprocessor:

```bash
cargo install mdbook-obsidian
```

Then, add the preprocessor to your `book.toml`:

```toml
[book]
authors = ["Jill Doe"]
language = "en"
multilingual = false
src = "src"
title = "My awesome Book"

# ADD THIS
[preprocessor.obsidian]

```

## Development

### How it works

The way this preprocessor works is primarily by using regexes to search for specific patterns like

```markdown
> [!CALLOUT_TYPE]
> CALLOUT_BODY
```

and then replacing it with the corresponding HTML code like

```html
<style>
  /* a bunch of CSS */
</style>
<div class="mdbook-obsidian-callouts mdbook-obsidian-callouts-{kind}">
  <p class="mdbook-obsidian-callouts-title">
    <span class="mdbook-obsidian-callouts-icon"></span>
    CALLOUT_TYPE
  </p>

  CALLOUT_BODY

</div>
```

### Expanding the preprocessor

The currently supported syntax is tracked in [this GitHub issue](https://github.com/GeckoEidechse/mdbook-obsidian/issues/1)

To add support for some currently unsupported syntax, expand the existing existing main render loop in `lib.rs`

```rust
/// Apply to all chapters
fn handle_chapter(chapter: &mut Chapter) -> Result<(), Error> {
    chapter.content = callouts::render(&chapter.content)?;
    // Add your additional syntax parsing here

    Ok(())
}
```

with a function that calls the corresponding parsing logic.

In your parsing logic, use regex or any other methods to scan for the specific pattern of the syntax you want to support and replace it with the corresponding HTML code.
