@pushd "%~dp0.." && setlocal

:: Missing symbols: memset, memcpy, memcmp, __chkstk, _fltused
::cargo +nightly build --example trivial
::@if ERRORLEVEL 1 goto :die

@cargo +nightly --version >NUL 2>NUL || goto :skip-nightly
    cargo +nightly build --manifest-path crates\no-std\Cargo.toml --release --example trivial -Zbuild-std=core --target=x86_64-pc-windows-msvc
    @if ERRORLEVEL 1 goto :die

    @setlocal
    @call "C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Auxiliary\Build\vcvars64.bat" >NUL
    @echo on
    @dumpbin /NOLOGO /IMPORTS crates\no-std\target\x86_64-pc-windows-msvc\release\examples\trivial.exe
    @endlocal

    crates\no-std\target\x86_64-pc-windows-msvc\release\examples\trivial.exe
:skip-nightly

cargo build --manifest-path crates/no-std/Cargo.toml --release --example trivial
@if ERRORLEVEL 1 goto :die

cargo test --workspace --no-default-features
@if ERRORLEVEL 1 goto :die

cargo test --workspace
@if ERRORLEVEL 1 goto :die

cargo build --examples
@if ERRORLEVEL 1 goto :die

target\debug\examples\max_sandbox.exe >NUL
@if ERRORLEVEL 1 goto :die



:die
@if ERRORLEVEL 1 echo BUILD ERRORS BUILD ERRORS BUILD ERRORS BUILD ERRORS BUILD ERRORS
@popd && endlocal && exit /b %ERRORLEVEL%
