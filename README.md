# `gm_filesystem`

This is a binary module that allows you to create/delete files in folders that lie in `garrysmod/`.

#### Adds functions to Lua:

* filesystem.Create(name, content)
* filesystem.Remove(name)
* filesystem.DirCreate(name)
* filesystem.DirRemove(name)
* filesystem.Exist(name)

# Build

All building guide [here](https://github.com/WilliamVenner/gmod-rs/blob/master/examples/my-first-binary-module/README.md); **(don't forget to put the nightly branch in rustup using the `rustup toolchain install nightly` command.)**

#### Linux:
* 32: rustup target add i686-unknown-linux-gnu --toolchain nightly
* 64: rustup target add x86_64-unknown-linux-gnu --toolchain nightly

# Todos

Add github workflows to build this library
