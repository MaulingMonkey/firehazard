Environment variables [`firehazard`] reads at build time.



I generally expect you to use a <code>.cargo\\[config.toml](https://doc.rust-lang.org/nightly/cargo/reference/config.html)</code> file to configure these settings, e.g.:

```toml
# your-project\.cargo\config.toml

[env]
FIREHAZARD_TYPE_CHECK_CAST_HANDLE           = "always"      # (default)
FIREHAZARD_TYPE_CHECK_OWNED_FROM_RAW        = "always"      # (default)
FIREHAZARD_TYPE_CHECK_BORROWED_FROM_RAW     = "always"      # (default)
```



# Assertion Control

These control potentially expensive checks for undefined behavior.
These checks cannot be relied upon for soundness, ergo *removing* these checks is sound:

*   Handle values can *and will* be reused - dangling or freed handles may not be reliably detected.
    This is not hypothetical: `#[isolate]` was introduced and applied to several unit tests in response to test failures on Windows 10.0.19045.5737 due to handle value reuse!

*   Handle type checks may rely on `Nt*` calls that may not be present for all past or future versions of Windows.

There are typically three possible values:

*   **always**  - default, always perform checks
*   debug       - perform checks if `cfg!(debug_assertions)`
*   never       - don't perform checks, even if `cfg!(debug_assertions)`

## `%FIREHAZARD_TYPE_CHECK_CAST_HANDLE%`
Attempt to validate a handle's type when casting between handle types.

This does *not* apply to *proven safe* conversions, e.g.:
*   <code>impl From&lt;[thread::OwnedHandle]&gt; for [handle::Owned]</code>

## `%FIREHAZARD_TYPE_CHECK_BORROWED_FROM_RAW%`
Attempt to validate a handle's type when using:
*   <code>Handle::[borrow_from_raw](FromLocalHandle::borrow_from_raw)\[[_nn](FromLocalHandle::borrow_from_raw_nn)\](&amp;handle)</code>
*   <code>Handle::[from_raw](FromLocalHandle::from_raw)\[[_nn](FromLocalHandle::from_raw_nn)\](handle)</code>   (if `Handle` is a borrowing handle.)
*   <code>Handle::[from_raw_handle](std::os::windows::io::FromRawHandle::from_raw_handle)(handle)</code>        (if `Handle` is a borrowing handle.)

## `%FIREHAZARD_TYPE_CHECK_OWNED_FROM_RAW%`
Attempt to validate a handle's type when using:
*   <code>Handle::[from_raw](FromLocalHandle::from_raw)\[[_nn](FromLocalHandle::from_raw_nn)\](handle)</code>   (if `Handle` is an owning handle.)
*   <code>Handle::[from_raw_handle](std::os::windows::io::FromRawHandle::from_raw_handle)(handle)</code>        (if `Handle` is an owning handle.)
