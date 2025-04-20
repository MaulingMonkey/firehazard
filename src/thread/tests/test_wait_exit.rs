#[cfg(std)] #[test] fn test_wait_exit() {
    use winapi::um::minwinbase::STILL_ACTIVE;
    use std::thread::*;

    let child = spawn(|| { sleep(core::time::Duration::from_millis(500)); unsafe { exit_thread(3) }; });
    let child = thread::OwnedHandle::from(child);

    assert!(is_thread_running(&child));
    assert_eq!(STILL_ACTIVE, get_exit_code_thread(&child).unwrap());

    assert_eq!(3, wait_for_thread(&child).unwrap());

    assert!(!is_thread_running(&child));
    assert_eq!(3, get_exit_code_thread(&child).unwrap());
}
