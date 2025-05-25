use std::backtrace::Backtrace;
use std::process::exit;
use to_vec::ToVec;

// unwind-agnostic variant of panic
#[macro_export]
macro_rules! abort_with_backtrace {
    ($($arg:tt)*) => {
        println!("\n\nabort_with_backtrace:");
        println!($($arg)*);
        error::print_backtrace_and_exit()
    }
}

pub fn print_backtrace_and_exit() -> ! {
    let backtrace = Backtrace::capture().to_string();
    let backtrace_lines = backtrace.lines().to_vec();
    let mut frames = Vec::new();
    for i in 0..backtrace_lines.len() / 2 {
        frames.push(format!("\n{}\n{}", backtrace_lines[i * 2], backtrace_lines[i * 2 + 1]));
    }
    let frames = frames.into_iter()
        .filter(|it| it.contains("fyrox"))
        .to_vec();
    // skip this frame
    let frames = &frames[1..];
    println!("\nRust backtrace:\n{}", frames.join("\n"));
    exit(666);
}