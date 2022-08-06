# Access Token APIs

## Types
Access tokens are referenced by `HANDLE`.  This is conceptually similar to a `Box<Arc<NtAccessToken>>`'s raw pointer.  Caveats:
*   Windows provides no actual RAII type.
*   The `Box` doesn't actually contain a raw pointer - instead, it contains a (generational?) index into some (OS-owned?) table.
*   The `Arc` is completely hidden away from the end user... and shared among processes?
*   The `NtAccessToken` is completely hidden away from the end user... and shared among processes?
*   `NtAccessToken` has interior mutability.

## Functions: Lifetime
*   `CloseHandle`:      Equivalent to `drop(Box::from_raw(...))`.
*   `OpenProcessToken`: Usually successful.  Roughly equivalent to `Box::new(Arc::clone(PROCESS_TOKEN))`: calling multiple times returns different handles that reference the same process token.
*   `OpenThreadToken`:  Usually fails with `GetLastError() == ERROR_NO_TOKEN` (only succeeds if thread has switched tokens?)
*   `DuplicateHandle`:  Shallow clone ala ``Box::new(Arc::clone(...))`: underlying token shares permission lists etc. with the original token handle.
*   `DuplicateTokenEx`: Deep clone ala `Box::new(Arc::new(NtAccessToken::clone(...)))`: underlying token has a new permission list that can be modified independently of the original token handle.
*   `DuplicateToken`:   Deep clone ala `Box::new(Arc::new(NtAccessToken::clone(...)))`?  I can't seem to modify the resulting token...
*   `GetCurrentProcessToken` - returns a **psuedo**-token.
*   `GetCurrentThreadToken` - returns a **psuedo**-token.
*   `GetCurrentThreadEffectiveToken` - returns a **psuedo**-token.

## Functions: Querying
*   `GetTokenInformation`
    *   Sometimes provides a smaller-than-type buffer (e.g. reads out 1 byte for "DWORD" booleanish value)
    *   Frequently needs a larger-than-type buffer (e.g. reads out a `TOKEN_USER`... but then also reads out more data to store the `SID` the `TOKEN_USER` has a pointer to.)
    *   Since the output contains pointers into itself, moving stack-owned buffers will break many outputs - typically better to read directly into heap allocs instead?
    *   Several token classes have overlapping information (e.g. `TokenGroupsAndPrivileges` contains all the information that `TokenPrivileges` does)

## Functions: Modification
*   `AdjustTokenPrivileges`: Enable, disable, or remove privileges, optionally returning the previous privileges.
*   `CreateRestrictedToken`: `DuplicateTokenEx` + `AdjustTokenPrivileges` + ...
