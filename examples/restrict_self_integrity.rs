use win32_security_playground::*;
use std::fs::File;

fn main() {
    // Medium: Can read+write
    let temp = std::env::var("TEMP").unwrap(); // C:\Users\{username}\AppData\Local\Temp\
    File::create(format!(r"{temp}\win32_security_playground_medium.txt")).unwrap();

    // Lower integrity
    let t = open_process_token(get_current_process(), token::ADJUST_DEFAULT).unwrap();
    t.set_integrity_level(sid::AndAttributes::new(sid!(S-1-16-0), 0)).unwrap();
    dbg!(open_process_token(get_current_process(), token::QUERY).unwrap().integrity_level().unwrap().label().sid);
    t.set_integrity_level(sid::AndAttributes::new(sid!(S-1-16-0x1000), 0)).unwrap_err(); // Can't raise integrity
    drop(t);

    // Untrusted: Can read, can't write
    File::create(format!(r"{temp}\win32_security_playground_untrusted.txt")).unwrap_err();
    File::open(format!(r"{temp}\win32_security_playground_medium.txt")).unwrap();
}
