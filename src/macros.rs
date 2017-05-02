//macro for indenting examples

#[macro_export]
macro_rules! xprintln {
    () => (print!("\n"));
    ($fmt:expr) =>
    ({
        print!("\t");
        print!(concat!($fmt, "\n"));
    });
    ($fmt:expr, $($arg:tt)*) =>
    ({
        print!("\t");
        print!(concat!($fmt, "\n"), $($arg)*);
    });
}
