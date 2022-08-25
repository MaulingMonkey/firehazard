@pushd "%~dp0.." && setlocal

@cargo check --workspace --all-targets
@if ERRORLEVEL 1 goto :die

:die
@popd && endlocal && exit /b %ERRORLEVEL%
