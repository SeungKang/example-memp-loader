# memp loader

**me**dge **mu**ltiplayer loader

A DLL written in Rust that loads the multiplayer mod DLL into Mirror's Edge when
the game starts. This DLL is loaded by using Phantom DLL hijacking techniques
and taking the name of the unused DLL "AgPerfMon.dll". After the multiplayer
mod DLL is loaded the memp loader DLL is deloade from the game.

[Mirror's Edge Multiplayer Mod](https://github.com/LucasOe/mmultiplayer)

Referenced in my blog post: [Chasing Ghosts: Phantom DLLs in Mirror's Edge](https://www.shonk.sh/posts/chasing-ghosts/)

## Build DLL

> Note: By default the `.cargo/config.toml` builds for windows 32-bit x86.

1. Install Rust through rustup
2. Install toolchain `rustup install stable-i686-pc-windows-msvc`
3. Add target `rustup target add i686-pc-windows-msvc`
4. Might need to do a `cargo clean`
5. Build with `cargo build`
6. File gets created in
   `PROJECT-PATH/target/i686-pc-windows-msvc/debug/AgPerfMon.dll`
7. Copy AgPerfMon.dll into a Dll search order directory

## Additional Work

- [without-dependencies](../../tree/without-dependencies) - same code as main
  branch but without any external dependencies
- [find-mp-dll-by-modules](../../tree/find-mp-dll-by-modules) - use mmultiplayer
  dll relative to loader dll
