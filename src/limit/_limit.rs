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

    pub const DEBUG_STRING  : usize = parse_limit_usize!(512, "FIREHAZARD_LIMIT_STACK_DEBUG_STRING" ) + 1;
    pub const PIPE_NAME     : usize = parse_limit_usize!(260, "FIREHAZARD_LIMIT_STACK_PIPE_NAME"    ) + 1;
    pub const PATH          : usize = parse_limit_usize!(260, "FIREHAZARD_LIMIT_STACK_PATH"         ) + 1;
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
