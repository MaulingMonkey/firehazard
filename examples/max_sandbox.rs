use win32_security_playground::*;

use winapi::um::processthreadsapi::STARTUPINFOW;
use winapi::um::winbase::*;
use winapi::um::winnt::SecurityImpersonation;

use std::os::windows::prelude::*;

fn main() {
    let t = open_process_token(get_current_process(), token::ALL_ACCESS).unwrap();
    let gap = t.groups_and_privileges().unwrap();
    let permissive = unsafe { create_restricted_token(&t, 0, None,             Some(gap.privileges()), Some(&gap.sids().iter().map(|s| sid::AndAttributes::new(s.sid, 0)).collect::<Vec<_>>()[..])) }.unwrap();
    let restricted = unsafe { create_restricted_token(&t, 0, Some(gap.sids()), Some(gap.privileges()), Some(&[sid::AndAttributes::new(sid!(S-1-0-0), 0)])) }.unwrap();
    let untrusted = sid::AndAttributes::new(sid!(S-1-16-0), 0);
    permissive.set_integrity_level(untrusted).unwrap();
    restricted.set_integrity_level(untrusted).unwrap();
    let permissive = unsafe { duplicate_token_ex(&permissive, token::ALL_ACCESS, None, SecurityImpersonation, token::Impersonation) };

    let exe = std::path::PathBuf::from(std::env::args_os().next().expect("args[0] / exe")).parent().unwrap().join("trivial.exe");
    let mut command_line = abistr::CStrBuf::<u16, 32768>::from_truncate(&exe.as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>());
    let mut si = STARTUPINFOW { lpDesktop: std::ptr::null_mut(), dwFlags: STARTF_UNTRUSTEDSOURCE, .. unsafe { std::mem::zeroed() } };
    si.cb = std::mem::size_of_val(&si) as u32;
    let pi = unsafe { create_process_as_user_w(&restricted, (), Some(command_line.buffer_mut()), None, None, false, CREATE_SEPARATE_WOW_VDM | CREATE_SUSPENDED, None, (), &si)}.unwrap();
    set_thread_token(&pi.thread, &permissive).unwrap();
    resume_thread(&pi.thread).unwrap();

    let exit = wait_for_process(pi.process).unwrap();
    assert!(exit == 0, "exit code: 0x{exit:08x} {}", LastError::from(exit).friendly());
}
