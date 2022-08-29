pub use sandbox_windows_ffi::job::*;
use sandbox_windows_ffi::*;
use winapi::um::winnt::*;
use std::mem::zeroed;



pub fn create() -> job::OwnedHandle {
    let mut job = create_job_object_a(None, ()).unwrap();
    relimit(&mut job, 1);

    // TODO: consider UserHandleGrantAccess to... do what, exactly?
    // https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject
    set_information_job_object(&mut job, JOBOBJECT_BASIC_UI_RESTRICTIONS { UIRestrictionsClass: 0
        | JOB_OBJECT_UILIMIT_DESKTOP            // Prevents processes associated with the job from creating desktops and switching desktops using the CreateDesktop and SwitchDesktop functions.
        | JOB_OBJECT_UILIMIT_DISPLAYSETTINGS    // Prevents processes associated with the job from calling the ChangeDisplaySettings function.
        | JOB_OBJECT_UILIMIT_EXITWINDOWS        // Prevents processes associated with the job from calling the ExitWindows or ExitWindowsEx function.
        | JOB_OBJECT_UILIMIT_GLOBALATOMS        // Prevents processes associated with the job from accessing global atoms. When this flag is used, each job has its own atom table.
        | JOB_OBJECT_UILIMIT_HANDLES            // Prevents processes associated with the job from using USER handles owned by processes not associated with the same job.
        | JOB_OBJECT_UILIMIT_READCLIPBOARD      // Prevents processes associated with the job from reading data from the clipboard.
        | JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS   // Prevents processes associated with the job from changing system parameters by using the SystemParametersInfo function.
        | JOB_OBJECT_UILIMIT_WRITECLIPBOARD     // Prevents processes associated with the job from writing data to the clipboard.
    }).unwrap();
    #[cfg(nope)] // TODO: the pointers in this type would require set_information_job_object to be `unsafe`, replace with a safer type
    set_information_job_object(&mut job, JOBOBJECT_SECURITY_LIMIT_INFORMATION {
        SecurityLimitFlags: 0
            | JOB_OBJECT_SECURITY_NO_ADMIN          // Prevents any process in the job from using a token that specifies the local administrators group.
            | JOB_OBJECT_SECURITY_RESTRICTED_TOKEN  // Prevents any process in the job from using a token that was not created with the CreateRestrictedToken function.
            // | JOB_OBJECT_SECURITY_FILTER_TOKENS    // Applies a filter to the token when a process impersonates a client. Requires at least one of the following members to be set: SidsToDisable, PrivilegesToDelete, or RestrictedSids.
            // | JOB_OBJECT_SECURITY_ONLY_TOKEN       // I don't have JobToken set - and would this prevent SetThreadToken ?
            ,
        .. unsafe { zeroed() }
    }).unwrap();
    // TODO: JOBOBJECT_END_OF_JOB_TIME_INFORMATION to hard-terminate the processes of the job?
    // TODO: JobObjectGroupInformation processor groups?
    // TODO: JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 limits?
    // TODO: JOBOBJECT_NET_RATE_CONTROL_INFORMATION to disable network?
    // TODO: JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION[_2] ?
    // TODO: JOBOBJECT_LIMIT_VIOLATION_INFORMATION ?
    // TODO: SetIoRateControlInformationJobObject ?
    job
}

pub fn relimit(job: &job::OwnedHandle, processes: u32) {
    set_information_job_object(job, JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
        BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION {
            LimitFlags: 0
                | JOB_OBJECT_LIMIT_ACTIVE_PROCESS
                | JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION
                | JOB_OBJECT_LIMIT_JOB_MEMORY
                | JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
                // | JOB_OBJECT_LIMIT_JOB_TIME // ?
                ,
            ActiveProcessLimit: processes,
            //PerJobUserTimeLimit: ..., // ?
            .. unsafe { zeroed() }
        },
        JobMemoryLimit: 4 * 1024*1024*1024, // 4 GiB
        .. unsafe { zeroed() }
    }).unwrap();
}