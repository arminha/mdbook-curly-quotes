# mdbook-curly-quotes

[![Build Status](https://travis-ci.org/arminha/mdbook-curly-quotes.svg?branch=master)](https://travis-ci.org/arminha/mdbook-curly-quotes)
[![build](https://github.com/arminha/mdbook-curly-quotes/workflows/build/badge.svg)](https://github.com/arminha/mdbook-curly-quotes/actions?query=workflow%3Abuild)
[![dependency status](https://deps.rs/repo/github/arminha/mdbook-curly-quotes/status.svg)](https://deps.rs/repo/github/arminha/mdbook-curly-quotes)
[![Crates.io](https://img.shields.io/crates/v/mdbook-curly-quotes)](https://crates.io/crates/mdbook-curly-quotes)

[mdBook](https://github.com/rust-lang/mdBook) preprocessor that replaces straight quotes with curly quotes, except within code blocks or code spans.

It does the same as the **curly-quotes** option of the mdBook HTML renderer. The only advantage is that it can be applied to any renderer.

## Usage

The following example configures mdbook-curly-quotes as a preprocessor for the epub renderer.

```toml
[book]
title = "Example book"
author = "John Doe"

# add the curly-quotes preprocessor
[preprocessor.curly-quotes]
# select renderers
renderer = ["epub"]

[output.epub]
```

More on configuring preprocessors can be found in the [mdBook Documentation](https://rust-lang.github.io/mdBook/format/config.html#configuring-preprocessors).
