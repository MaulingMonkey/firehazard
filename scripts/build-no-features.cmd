@pushd "%~dp0.." && setlocal
@set CARGO_TARGET_DIR=target\no-features



cargo test --workspace --no-default-features
@if ERRORLEVEL 1 goto :die



:die
@if ERRORLEVEL 1 echo BUILD ERRORS BUILD ERRORS BUILD ERRORS BUILD ERRORS BUILD ERRORS
@popd && endlocal && exit /b %ERRORLEVEL%
