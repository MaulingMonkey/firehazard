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



# Stack Limits

These control the maximum size of stack buffers to use when converting from e.g. `&str` (UTF-8, not `\0`-terminated) to native (UTF-16ish, `\0`-terminated) strings.
Heap allocation will typically be attempted if these limits are exceeded.

## `%FIREHAZARD_LIMIT_STACK_APP_CONTAINER_*%`

* `%FIREHAZARD_LIMIT_STACK_APP_CONTAINER_NAME%`             defaults to   **64**
* `%FIREHAZARD_LIMIT_STACK_APP_CONTAINER_DISPLAY_NAME%`     defaults to  **512**
* `%FIREHAZARD_LIMIT_STACK_APP_CONTAINER_DESCRIPTION%`      defaults to **2048**

These limits are based on [the documented limits](https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-createappcontainerprofile) and affect the following functions:

*   [`create_app_container_profile`]
*   [`delete_app_container_profile`]
*   [`derive_app_container_sid_from_app_container_name`]
*   ~~`derive_restricted_app_container_sid_from_app_container_sid_and_restricted_name`~~

## `%FIREHAZARD_LIMIT_STACK_DEBUG_STRING%`
default: **512** (limit before truncation by some versions of Windows?)

Affected functions include:
*   [`debug::output_debug_string`]

## `%FIREHAZARD_LIMIT_STACK_PATH%`
default: **260** (256 character path + 1 '.' + 3 character extension?)

Affected functions include:
*   [`create_file`]
*   [`create_process`]
*   [`create_process_as_user`]
*   [`get_file_attributes`]
*   [`convert_string_sid_to_sid`]

There are also derived limits based on this:

*   `%FIREHAZARD_LIMIT_STACK_CAPABILITY_NAME%`, affecting [`derive_capability_sids_from_name`]
*   `%FIREHAZARD_LIMIT_STACK_COMPUTER_NAME%`,   affecting [`lookup_privilege_value`]
*   `%FIREHAZARD_LIMIT_STACK_DESKTOP_NAME%`,    affecting [`create_desktop`], [`open_desktop`]
*   `%FIREHAZARD_LIMIT_STACK_JOB_NAME%`,        affecting [`create_job_object`], [`open_job_object`]
*   `%FIREHAZARD_LIMIT_STACK_PIPE_NAME%`,       affecting <code>pipe::named::{[call](pipe::named::call), [create](pipe::named::create), [wait](pipe::named::wait)}</code>
*   `%FIREHAZARD_LIMIT_STACK_PRIVILEGE_NAME%`,  affecting [`derive_capability_sids_from_name`]
*   `%FIREHAZARD_LIMIT_STACK_WINSTA_NAME%`,     affecting [`create_window_station`], [`open_window_station`]

## `%FIREHAZARD_LIMIT_STACK_SID_STRING%`
default: **256** (SID strings approaching 100 length are common, 256 is probably sufficient.)

Affected functions include:
*   [`convert_string_sid_to_sid`]
