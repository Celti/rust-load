extern crate getopts;
extern crate libc;

use getopts::Options;
use libc::{c_double, c_int};
use std::mem::transmute;
use std::env;

#[cfg(unix)]
extern {
    fn getloadavg(loadavg: *mut c_double, nelem: c_int) -> c_int;
}

fn print_load_average(index: usize) {
    let mut avg: [c_double; 3] = [0.0; 3];
    let loads: i32 = unsafe { transmute(getloadavg(avg.as_mut_ptr(), 3)) };

    if loads == -1 {
        print!("\n");
    } else {
        println!("{:.2}", avg[index]);
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut index: usize = 0;

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help");
    opts.optflag("1", "one", "print one minute load average (default)");
    opts.optflag("5", "five", "print five minute load average");
    opts.optflag("", "fifteen", "print fifteen minute load average");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(_) => {
            print_usage(&program, opts);
            return;
        },
    };

    if matches.opt_present("help") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("one") {
        index = 0;
    }

    if matches.opt_present("five") {
        index = 1;
    }

    if matches.opt_present("fifteen") {
        index = 2;
    }

    print_load_average(index);
}
