# ❌ Allocations

This crate is likely not yet 100% sound with regards to allocations made by
the underlying Win32 C APIs.  Unsoundness may arise from:
*   Allocation size calculations overflowing a `u32` or `usize` and under-allocating.
*   Allocation failing and not gracefully handling the returned `nullptr`s.

While examples of such unsoundness are arguably bugs belonging to the C layer,
rather than this crate, it's within scope to request additional parameter
validation to avoid such bugs.  I may introduce `unsafe` fns to opt-*out* of new
validation such as:
*   `limits::set_max_sids(...)`
*   `validation::set_access_flags_validation(|...| ...)`

...where defaults are conservative in trying to ban only legitimately invalid behavior.
While this can guard against overflows, nothing can reasonably fully guard against
non-overflowing allocation failure - nothing's stopping you from exhausting your
entire process's free virtual memory address space with `Box`es or `Vec`s.

# ⚠️ Interior mutability

APIs modifying `HANDLE`s currently take them by `&self`.  This is, technically,
currently, sound - as the "pointer" itself isn't modified - but this could become
quite sketchy if they ever become `Send` or `Sync` and get modified from multiple
threads.  Also it just looks gross, so I should seriously consider "fixing" this.

# ⚠️ Handle validity

Win32 APIs are generally pretty good about failing with e.g. `ERROR_INVALID_HANDLE`
instead of otherwise misbehaving when passed invalid `HANDLE` values, or `HANDLE`s
to the wrong type of object.  Additionally, most methods accepting raw `HANDLE`s
are currently `unsafe`.  That said, for additional protection, you should
consider using [`PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY`] to more strictly
check/abort in the case of invalid `HANDLE`s.

[`PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY`]:  https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_strict_handle_check_policy
