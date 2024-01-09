pub enum ExitCode {
    // Linux exit codes at /usr/include/sysexits.h
    OK = 0,       /* successful termination */
    USAGE = 64,   /* command line usage error */
    DATAERR = 65, /* data format error */
    NOINPUT = 66, /* cannot open input */
    // NOUSER = 67,      /* addressee unknown */
    // NOHOST = 68,      /* host name unknown */
    // UNAVAILABLE = 69, /* service unavailable */
    SOFTWARE = 70, /* internal software error */
    // OSERR = 71,       /* system error (e.g., can't fork) */
    // OSFILE = 72,      /* critical OS file missing */
    // CANTCREAT = 73,   /* can't create (user) output file */
    IOERR = 74, /* input/output error */
                // TEMPFAIL = 75,    /* temp failure; user is invited to retry */
                // PROTOCOL = 76,    /* remote error in protocol */
                // NOPERM = 77,      /* permission denied */
                // CONFIG = 78,      /* configuration error */
}

pub static mut TOKENIZER_ERROR: bool = false;
pub static mut INTERPRETER_ERROR: bool = false;

pub fn code_error() -> ExitCode {
    unsafe {
        if TOKENIZER_ERROR {
            ExitCode::DATAERR
        } else if INTERPRETER_ERROR {
            ExitCode::SOFTWARE
        } else {
            ExitCode::OK
        }
    }
}

pub fn clear_errors() {
    unsafe {
        TOKENIZER_ERROR = false;
        INTERPRETER_ERROR = false;
    }
}

pub fn tokenizer_error(line: usize, message: String) {
    println!("[line {}] Error: {}", line, message);
    unsafe { TOKENIZER_ERROR = true }
}
