//! \[~~microsoft.com~~\]
//! Various limits etc.

macro_rules! parse_limit_usize {
    ( $default:expr, $env:expr ) => {{
        use $crate::limit::*;
        match parse_limit_usize($default, option_env!($env)) {
            Ok(v)                           => v,
            Err(ParseError::Overflow)       => panic!(concat!("expected a `usize` value for `%", $env, "%`, but the value exceeded `usize::MAX`")),
            Err(ParseError::NotAnInteger)   => panic!(concat!("expected a `usize` value for `%", $env, "%`, but the value wasn't an integer")),
        }
    }};
}

pub mod stack {
    // note: many sneaky +1s to include a `\0`

    // limits documented by https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-createappcontainerprofile
    pub const APP_CONTAINER_NAME            : usize = parse_limit_usize!(  64, "FIREHAZARD_LIMIT_STACK_APP_CONTAINER_NAME"          ) + 1;
    pub const APP_CONTAINER_DISPLAY_NAME    : usize = parse_limit_usize!( 512, "FIREHAZARD_LIMIT_STACK_APP_CONTAINER_DISPLAY_NAME"  ) + 1;
    pub const APP_CONTAINER_DESCRIPTION     : usize = parse_limit_usize!(2048, "FIREHAZARD_LIMIT_STACK_APP_CONTAINER_DESCRIPTION"   ) + 1;

    pub const DEBUG_STRING                  : usize = parse_limit_usize!( 512, "FIREHAZARD_LIMIT_STACK_DEBUG_STRING"                ) + 1; // n.b. some platforms start truncating beyond 512 characters

    pub const PATH                          : usize = parse_limit_usize!( 260, "FIREHAZARD_LIMIT_STACK_PATH"                        ) + 1; // ≈ MAX_PATH = 256 + 1 for '.' + 3 for extension
    pub const CAPABILITY_NAME               : usize = parse_limit_usize!(PATH-1, "FIREHAZARD_LIMIT_STACK_CAPABILITY_NAME"           ) + 1; // vaguely path-like?
    pub const COMPUTER_NAME                 : usize = parse_limit_usize!(PATH-1, "FIREHAZARD_LIMIT_STACK_COMPUTER_NAME"             ) + 1; // vaguely path-like?
    pub const DESKTOP_NAME                  : usize = parse_limit_usize!(PATH-1, "FIREHAZARD_LIMIT_STACK_DESKTOP_NAME"              ) + 1; // vaguely path-like?
    pub const JOB_NAME                      : usize = parse_limit_usize!(PATH-1, "FIREHAZARD_LIMIT_STACK_JOB_NAME"                  ) + 1; // vaguely path-like?
    pub const PIPE_NAME                     : usize = parse_limit_usize!(PATH-1, "FIREHAZARD_LIMIT_STACK_PIPE_NAME"                 ) + 1; // vaguely path-like?
    pub const PRIVILEGE_NAME                : usize = parse_limit_usize!(PATH-1, "FIREHAZARD_LIMIT_STACK_PRIVILEGE_NAME"            ) + 1; // vaguely path-like?
    pub const WINSTA_NAME                   : usize = parse_limit_usize!(PATH-1, "FIREHAZARD_LIMIT_STACK_WINSTA_NAME"               ) + 1; // vaguely path-like?

    pub const SID_STRING                    : usize = parse_limit_usize!( 256, "FIREHAZARD_LIMIT_STACK_SID_STRING"                  ) + 1; // 100 is common, 256 is more than double
}



pub(crate) const fn parse_limit_usize(default: usize, s: Option<&str>) -> Result<usize, ParseError> {
    let Some(s) = s else { return Ok(default) };
    let mut v = 0_usize;
    let mut s = s.as_bytes();
    while let [b, rest @ ..] = s {
        let b = match *b {
            b @ b'0' ..= b'9'   => (b - b'0') as usize,
            _                   => return Err(ParseError::NotAnInteger),
        };

        let Some(v0) = v.checked_mul(10) else { return Err(ParseError::Overflow) };
        let Some(vb) = v0.checked_add(b) else { return Err(ParseError::Overflow) };
        v = vb;
        s = rest;
    }

    Ok(v)
}

pub(crate) enum ParseError {
    Overflow,
    NotAnInteger,
}
