@pushd "%~dp0.." && setlocal
@set CARGO_TARGET_DIR=target\i686



cargo test --workspace --target=i686-pc-windows-msvc
@if ERRORLEVEL 1 goto :die



:die
@if ERRORLEVEL 1 echo BUILD ERRORS BUILD ERRORS BUILD ERRORS BUILD ERRORS BUILD ERRORS
@popd && endlocal && exit /b %ERRORLEVEL%
