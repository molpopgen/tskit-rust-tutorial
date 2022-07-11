# tskit-rust tutorial

Source code for the [tskit-rust](https://tskit.dev/tskit_rust) tutorial.

## Setup

* Define a trivial rust library that simply re-exports the `tskit` API.
  * Build `tskit` with all cargo features so that we can use them in examples.
* Code examples for the book go in `tests/` using anchors.
* Book contents are markdown rendered with [mdbook](https://rust-lang.github.io/mdBook/).
