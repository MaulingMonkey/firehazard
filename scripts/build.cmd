@pushd "%~dp0.." && setlocal

:: Missing symbols: memset, memcpy, memcmp, __chkstk, _fltused
::cargo +nightly build --no-default-features --example trivial
::@if ERRORLEVEL 1 goto :die

cargo +nightly build --release --no-default-features --example trivial
@if ERRORLEVEL 1 goto :die

cargo test
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
