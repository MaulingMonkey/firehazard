//! Use job objects to somewhat limit what a process can do.
//!
//! Note that jobs aren't inescapeable:
//! *   [project-zero: Blog: In-Console-Able (2015)](https://googleprojectzero.blogspot.com/2015/05/in-console-able.html)
//! *   [project-zero: Issue 213: Windows: Console Driver Job Object Process Limit Bypass](https://bugs.chromium.org/p/project-zero/issues/detail?id=213&redir=1)

pub use firehazard::job::*;
use firehazard::*;



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
    set_information_job_object(&mut job, job::object::EndOfJobTimeInformation {
        // default behavior, but doesn't hurt to be explicit?
        // in case parent job had a different policy perhaps?
        end_of_job_time_action: job::object::TERMINATE_AT_END_OF_JOB,
    }).unwrap();
    // TODO: JobObjectGroupInformation processor groups?
    // TODO: JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 limits?
    set_information_job_object(&mut job, job::object::NetRateControlInformation {
        max_bandwidth:  0, // limit network egress
        control_flags:  job::object::NET_RATE_CONTROL_ENABLE | job::object::NET_RATE_CONTROL_MAX_BANDWIDTH,
        .. Default::default()
    }).unwrap();
    // TODO: JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION[_2] ?
    // TODO: JOBOBJECT_LIMIT_VIOLATION_INFORMATION ?
    // TODO: SetIoRateControlInformationJobObject ?
    job
}

pub fn relimit(job: &job::OwnedHandle, processes: u32) {
    set_information_job_object(job, job::object::ExtendedLimitInformation {
        basic_limit_information: job::object::BasicLimitInformation {
            limit_flags: ()
                | job::object::limit::ACTIVE_PROCESS
                | job::object::limit::DIE_ON_UNHANDLED_EXCEPTION
                | job::object::limit::JOB_MEMORY
                | job::object::limit::KILL_ON_JOB_CLOSE
                // | job::object::limit::JOB_TIME // ?
                ,
            active_process_limit: processes,
            // per_job_user_time_limit: ..., // ?
            .. Default::default()
        },
        job_memory_limit: 4 * 1024*1024*1024, // 4 GiB
        .. Default::default()
    }).unwrap();
}
