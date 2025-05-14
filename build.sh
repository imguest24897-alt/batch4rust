# This is for Unix operating systems. If you use Windows, please use the "build.bat" file.

if [[ "$1" == "-r" ]]; then
    cargo build --target x86_64-pc-windows-gnu --release
else
    cargo build --target x86_64-pc-windows-gnu
fi