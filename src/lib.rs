//! # Tacky Borders Logger
//!
//! This module provides macros for logging messages at different levels (debug, info, warn, error).
//! Each macro dynamically includes the function name where it is invoked. These macros are useful for
//! logging contextual information along with formatted messages, and they automatically capture the
//! function name at the point where the macro is called.
//!
//! # Usage
//! You can use these macros to log messages with the current function name included. The function name
//! is extracted dynamically using the `function_name!()` macro, which returns the name of the function
//! where it is invoked.
//!
//! The macros can be used to log messages at different severity levels:
//!
//! - `trace!` - Logs detailed, low-level information for tracing program execution. Typically used for understanding the flow of execution, especially for debugging intricate issues. This log level provides the most granular level of detail, useful for tracking function calls, variable states, or intricate interactions between different parts of the program.
//! - `debug!` - Logs detailed information, typically for development or debugging purposes.
//! - `info!` - Logs general information messages.
//! - `warn!` - Logs warnings about potential issues that aren't necessarily errors.
//! - `error!` - Logs error messages, typically when something goes wrong in the program.
//!
//! # Example
//! ```rust
//! fn example_function() {
//!     trace!("This is a trace message.");
//!     debug!("This is a debug message.");
//!     info!("This is an info message.");
//!     warn!("This is a warning message.");
//!     error!("This is an error message.");
//! }
//! ```
//!
//! # Function Name Extraction
//! The `function_name!()` macro uses the Rust type system to extract the name of the function dynamically.
//! It utilizes `std::any::type_name::<T>()` to obtain the type name of the current function, and then processes
//! it to extract just the function's name. This function name is included in the log messages to provide more
//! context about where the log entry was made.
//!
//! The macros work by formatting a message and appending the function name at the end, which helps in tracing
//! logs and identifying which function generated a particular log message.

/// Macro to extract the name of the current function as a string.
///
/// This macro returns the name of the function where it is invoked. It uses
/// `std::any::type_name` to obtain the function's type name and then extracts
/// the function's name by processing the string. It assumes the function is
/// defined in the current scope.
///
/// # Example
/// ```rust
/// fn example_function() {
///     let fn_name = function_name!();
///     println!("{}", fn_name); // Prints "example_function"
/// }
/// ```
/// # Notes
/// - This macro works by exploiting Rust's type system to extract the name
///   of the current function.
/// - It trims the `::f` suffix and handles any nested modules if present.
#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> String {
            std::any::type_name::<T>().to_string()
        }
        let name = type_name_of(f);
        let func_name = &name[..name.len() - 3]; // Remove the `::f`
        let func_name_split: Vec<String> = func_name.split("::").map(|s| s.to_string()).collect();
        let last_part = func_name_split.last().unwrap();
        last_part.clone()
    }};
}

/// Macro to log debug-level messages with the current function name.
///
/// This macro logs a debug message along with the name of the function
/// where the macro is invoked. It also includes any formatted arguments.
///
/// # Example
/// ```rust
/// fn some_function() {
///     debug!("This is a debug message.");
///     // Logs: "This is a debug message. [fn some_function]"
/// }
/// ```
/// # Notes
/// - It appends the function name dynamically for context.
/// - Useful for detailed logging during development.
#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => ({
        let fn_name = function_name!();
        let formatted_message = format!("{} [fn {}]", format_args!($($arg)*), fn_name);
        log::debug!("{}", formatted_message);
    });
}

/// Macro to log info-level messages with the current function name.
///
/// This macro logs an info-level message with the current function name.
/// Like the `debug!` macro, it can format and display additional arguments.
///
/// # Example
/// ```rust
/// fn example_function() {
///     info!("This is an info message.");
///     // Logs: "This is an info message. [fn example_function]"
/// }
/// ```
/// # Notes
/// - This macro provides a simple way to log information along with the
///   function name.
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ({
        let fn_name = function_name!();
        let formatted_message = format!("{} [fn {}]", format_args!($($arg)*), fn_name);
        log::info!("{}", formatted_message);
    });
}

/// Macro to log error-level messages with the current function name.
///
/// This macro logs an error-level message with the name of the function
/// where the macro is invoked. It includes any formatted arguments passed.
///
/// # Example
/// ```rust
/// fn another_function() {
///     error!("An error occurred!");
///     // Logs: "An error occurred! [fn another_function]"
/// }
/// ```
/// # Notes
/// - Used for logging error messages with context about where they occurred.
#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => ({
        let fn_name = function_name!();
        let formatted_message = format!("{} [fn {}]", format_args!($($arg)*), fn_name);
        log::error!("{}", formatted_message);
    });
}

/// Macro to log warning-level messages with the current function name.
///
/// This macro logs a warning-level message with the name of the function
/// where it is called. It can also include formatted arguments.
///
/// # Example
/// ```rust
/// fn some_function() {
///     warn!("This is a warning message.");
///     // Logs: "This is a warning message. [fn some_function]"
/// }
/// ```
/// # Notes
/// - This macro allows for logging warnings while automatically appending
///   the function name.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => ({
        let fn_name = function_name!();
        let formatted_message = format!("{} [fn {}]", format_args!($($arg)*), fn_name);
        log::warn!("{}", formatted_message);
    });
}

/// Macro to log trace-level messages with the current function name.
///
/// This macro logs a trace-level message with the name of the function
/// where it is called. It can also include formatted arguments.
///
/// # Example
/// ```rust
/// fn any_function() {
///     warn!("This is a trace message.");
///     // Logs: "This is a trace message. [fn any_function]"
/// }
/// ```
/// # Notes
/// - This macro allows for logging traces while automatically appending
///   the function name.
#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => ({
        let fn_name = function_name!();
        let formatted_message = format!("{} at [fn {}]", format_args!($($arg)*), fn_name);
        log::trace!("{}", formatted_message);
    });
}
