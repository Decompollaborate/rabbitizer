# rab-disasmdis-web

Disassemble MIPS instructions directly in your browser, using the power of
rabbitizer and Rust.

This site is available at <https://decompollaborate.github.io/rabbitizer>.

## Running

disasmdis-web can be built and hosted locally, mainly for local development.

### Dependencies

This site is built as a WASM application, meaning we need the WASM Rust target
installed. I recommend using [`trunk`](https://trunkrs.dev/) to serve the
application locally.

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

### Serving

In your terminal navigate into the `rab-disasmdis-web` folder (i.e.
`cd src/rab-disasmdis-web`) and run `trunk`:

```bash
trunk serve --release --open
```

This will get build the aplication, and open it in your default browser. `trunk`
will watch for changes in the code and rebuild if any is made.
