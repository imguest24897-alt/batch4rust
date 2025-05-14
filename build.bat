@echo off
REM This is for NT-based Windows operating systems. If you use Unix, please use the "build.sh" file.

if "%1"=="-r" (
    cargo build --release
) else (
    cargo build
)