@pushd "%~dp0.." && setlocal

cargo test
@if ERRORLEVEL 1 goto :die

cargo build --examples --bin tests
@if ERRORLEVEL 1 goto :die

target\debug\examples\trivial.exe
@if ERRORLEVEL 1 goto :die

target\debug\examples\max_sandbox.exe >NUL 2>NUL
@if ERRORLEVEL 1 goto :die

target\debug\tests.exe
@if ERRORLEVEL 1 goto :die



:die
@popd && endlocal && exit /b %ERRORLEVEL%
