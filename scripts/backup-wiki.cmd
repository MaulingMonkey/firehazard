@pushd "%~dp0.." && setlocal
git fetch github-wiki
@popd && endlocal && exit /b %ERRORLEVEL%
