@pushd "%~dp0.." && setlocal

@cargo check --workspace
@if ERRORLEVEL 1 goto :die

:die
@popd && endlocal && exit /b %ERRORLEVEL%
