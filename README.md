# Tacky Borders Logger

This module provides macros for logging messages at different levels (`trace!`, `debug!`, `info!`, `warn!`, `error!`). Each macro dynamically includes the function name where it is invoked, making it easy to trace logs and understand the context in which they were generated. These macros are useful for logging contextual information along with formatted messages.

## Features
- Logs messages at various severity levels.
- Dynamically includes the function name in the log messages.
- Provides rich context for tracing and debugging the execution of the program.

## Usage

You can use these macros to log messages with the current function name included. The function name is extracted dynamically using the `function_name!()` macro, which returns the name of the function where it is invoked.

### Macros

#### `trace!`

Logs detailed, low-level information for tracing program execution. Typically used for understanding the flow of execution, especially for debugging intricate issues. This log level provides the most granular level of detail, useful for tracking function calls, variable states, or intricate interactions between different parts of the program.

#### `debug!`

Logs detailed information, typically for development or debugging purposes. This log level provides detailed information but is generally used in a development context to trace the execution of the program and inspect variable states.

#### `info!`

Logs general information messages. These are typically used to log normal operation information, such as the completion of an action or important milestones in the program's flow.

#### `warn!`

Logs warnings about potential issues that aren't necessarily errors. These messages help indicate situations that may require attention but do not break the program's execution.

#### `error!`

Logs error messages, typically when something goes wrong in the program. These messages indicate when a problem has occurred that requires immediate attention.

## Example

```rust
fn example_function() {
    trace!("This is a trace message.");
    debug!("This is a debug message.");
    info!("This is an info message.");
    warn!("This is a warning message.");
    error!("This is an error message.");
}
```

In this example, each log message will include the function name where the macro was invoked (e.g., `example_function`).

## Function Name Extraction
The `function_name!()` macro uses Rust's type system to extract the name of the function dynamically. It utilizes `std::any::type_name::<T>()` to obtain the type name of the current function, and then processes it to extract just the function's name. This function name is included in the log messages to provide more context about where the log entry was made.

The macros work by formatting a message and appending the function name at the end, which helps in tracing logs and identifying which function generated a particular log message.

## Example with Logging Levels

```rust
fn some_function() {
    debug!("This is a debug message.");
    info!("This is an info message.");
    warn!("This is a warning message.");
    error!("This is an error message.");
}
```

Each log entry generated will include the relevant log level and function name, e.g.,:

-   `debug: This is a debug message. [fn some_function]`
-   `info: This is an info message. [fn some_function]`
-   `warn: This is a warning message. [fn some_function]`
-   `error: This is an error message. [fn some_function]`

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
