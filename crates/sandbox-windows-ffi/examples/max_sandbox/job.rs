pub use firehazard::job::*;
use firehazard::*;
use winapi::um::winnt::*;
use std::mem::zeroed;



pub fn create() -> job::OwnedHandle {
    let mut job = create_job_object_a(None, ()).unwrap();
    relimit(&mut job, 1);

    // TODO: consider UserHandleGrantAccess to... do what, exactly?
    // https://docs.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-setinformationjobobject
    set_information_job_object(&mut job, job::object::BasicUiRestrictions { ui_restrictions_class: ()
        | job::object::uilimit::DESKTOP            // Prevents processes associated with the job from creating desktops and switching desktops using the CreateDesktop and SwitchDesktop functions.
        | job::object::uilimit::DISPLAYSETTINGS    // Prevents processes associated with the job from calling the ChangeDisplaySettings function.
        | job::object::uilimit::EXITWINDOWS        // Prevents processes associated with the job from calling the ExitWindows or ExitWindowsEx function.
        | job::object::uilimit::GLOBALATOMS        // Prevents processes associated with the job from accessing global atoms. When this flag is used, each job has its own atom table.
        | job::object::uilimit::HANDLES            // Prevents processes associated with the job from using USER handles owned by processes not associated with the same job.
        | job::object::uilimit::READCLIPBOARD      // Prevents processes associated with the job from reading data from the clipboard.
        | job::object::uilimit::SYSTEMPARAMETERS   // Prevents processes associated with the job from changing system parameters by using the SystemParametersInfo function.
        | job::object::uilimit::WRITECLIPBOARD     // Prevents processes associated with the job from writing data to the clipboard.
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
