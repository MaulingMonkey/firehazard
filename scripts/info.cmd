:: Run as separate stage in github CI to show tool/os versions for context
@rustc -V >NUL || set PATH=%PATH%;%USERPROFILE%\.cargo\bin
rustc -V
cargo -V
ver
