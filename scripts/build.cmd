@cargo +nightly --version >NUL 2>NUL && call "%~dp0.\build-nightly.cmd"
@if ERRORLEVEL 1 exit /b %ERRORLEVEL%

@call "%~dp0.\build-no-features.cmd"
@if ERRORLEVEL 1 exit /b %ERRORLEVEL%

@call "%~dp0.\build-basic.cmd"
@if ERRORLEVEL 1 exit /b %ERRORLEVEL%

@echo BUILD SUCCESS BUILD SUCCESS BUILD SUCCESS BUILD SUCCESS BUILD SUCCESS
