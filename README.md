Remove a lot of clutter and boilerplate when using `std::process::Command`.

## Macros

Use the `cmd!` macro to create plain `std::process:Command` structs, use `exec!` and `run!` to execute them directly via `status()` and `output()` respectively
```rust
run!(echo "Hello World!");
# equivalent to
Command::new("echo").arg("Hello World1").output();
```

All arguments will be treated as whitespace separated strings by default. If your argument includes whitespace, wrap it in quotes as shown above.

To use an existing variable as an arg, wrap its name in braces. You can mix arguments of different types (unlike the `.args()` method).

```rust
run!(echo (some_str) (some_string) (some_path));
# equivalent to
Command::new("echo").arg(some_str).arg(some_string).arg(some_path).output();
```

If your variable is iterable you can expand it using `..`

```rust
run!(echo Hello (worlds..))
```

If your argument is optional you can use `?` to only append it if it is `Some`
```rust
let name = Some("Steve");
run!(echo Hello (name ?));
```

If you include a literal inside the same braces the literal will also only be included if the value is `Some` (use this for flags).
This also works for iterables.
```rust
let packages = vec!["cowsay", "emacs"];
run!(echo Installing ("-p" packages ?));
```



Use the `args!` macro to append arguments to an exist `Command` using the same syntax as `cmd!`
```rust
fn my_function(mut cmd: Command) -> Command {
    cmd.args(args!(some new args here));
    cmd
}
```

## Extensions

Use the `cmd_ok()` method on the return value of `status()` or `output()` to create an `Err()` on _both_ IO failure and any exit code except `0`, the latter will include the contents of stderr if they are available.

```rust
run!(echo test).cmd_ok()?
```

Use `opt_arg` to pass optional values to your Command and only use them if they are `Some`.

Use `output.stdout()`, `output.stderr()`, `output.stdout_lossy()` and `output.stderr_lossy()` instead of `String::from_utf8(output.stdout)`.