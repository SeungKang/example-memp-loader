# memp loader

**me**dge **mu**ltiplayer loader

A DLL written in Rust that loads the multiplayer mod DLL into Mirror's Edge when
the game starts. This DLL is loaded by using Phantom DLL hijacking techniques
and taking the name of the unused DLL "AgPerfMon.dll". After the multiplayer
mod DLL is loaded the memp loader DLL is deloade from the game.

[Mirror's Edge Multiplayer Mod](https://github.com/LucasOe/mmultiplayer)

Referenced in my blog post: TODO add link to blog post

## Build DLL

1. Install Rust through rustup
2. Install toolchain `rustup install stable-i686-pc-windows-msvc`
3. Add target `rustup target add i686-pc-windows-msvc`
4. Might need to do a `cargo clean`
5. Build with `cargo build`
6. File gets created in
   `PROJECT-PATH/target/i686-pc-windows-msvc/debug/AgPerfMon.dll`
7. Copy AgPerfMon.dll into a Dll search order directory
