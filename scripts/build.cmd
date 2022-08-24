@pushd "%~dp0.." && setlocal

:: Missing symbols: memset, memcpy, memcmp, __chkstk, _fltused
::cargo +nightly build --no-default-features --example trivial
::@if ERRORLEVEL 1 goto :die

@cargo +nightly --version >NUL 2>NUL || goto :skip-nightly
    cargo +nightly build --release --no-default-features --example trivial -Zbuild-std --target=x86_64-pc-windows-msvc
    @if ERRORLEVEL 1 goto :die

    @setlocal
    @call "C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Auxiliary\Build\vcvars64.bat" >NUL
    @echo on
    @dumpbin /NOLOGO /IMPORTS target\x86_64-pc-windows-msvc\release\examples\trivial.exe
    @endlocal
:skip-nightly

cargo test --features std --workspace
@if ERRORLEVEL 1 goto :die

cargo test --workspace
@if ERRORLEVEL 1 goto :die

cargo build --examples
@if ERRORLEVEL 1 goto :die

cargo build --features std --bin tests
@if ERRORLEVEL 1 goto :die

target\debug\examples\trivial.exe
@if ERRORLEVEL 1 goto :die

target\debug\examples\max_sandbox.exe >NUL 2>NUL
@if ERRORLEVEL 1 goto :die

target\debug\tests.exe
@if ERRORLEVEL 1 goto :die



:die
@popd && endlocal && exit /b %ERRORLEVEL%
