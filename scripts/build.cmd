@cargo +nightly --version >NUL 2>NUL || goto :skip-nightly
@   call "%~dp0.\build-nightly.cmd"                                     || goto :err
:skip-nightly
@call "%~dp0.\build-no-features.cmd"                                    || goto :err
@call "%~dp0.\build-basic.cmd"                                          || goto :err
@call "%~dp0.\build-i686.cmd"                                           || goto :err

@echo BUILD SUCCESS BUILD SUCCESS BUILD SUCCESS BUILD SUCCESS BUILD SUCCESS
:err
@exit /b %ERRORLEVEL%
