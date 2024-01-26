#[cfg(feature = "ok")]
pub mod ok;
#[cfg(feature = "opt_arg")]
pub mod opt_arg;
#[cfg(feature = "output")]
pub mod output;

pub mod internal {
    #[macro_export]
    macro_rules! cmd_partial {
        ((var($e:ident))) => {
            std::env::var(stringify!($e)).unwrap_or_default()
        };
        ($e:ident) => {
            stringify!($e)
        };
        ($e:expr) => {
            #[allow(unused_parens)]
            $e
        };
        ($e:tt) => {
            stringify!($e)
        };
    }

    #[macro_export]
    macro_rules! arg {
        ($x:ident; (var($e:ident))) => {
            $x.arg(std::env::var(stringify!($e)).unwrap_or_default())
        };
        ($x:ident; ($f:tt $e:tt?)) => {{
            #[cfg(feature = "opt_arg")]
            let x = $x.opt_arg(($crate::internal::cmd_partial!($f), $e));
            #[cfg(not(feature = "opt_arg"))]
            let x = compile_error!("Requires 'opt_arg' feature");
            x
        }};
        ($x:ident; ($e:tt?)) => {{
            #[cfg(feature = "opt_arg")]
            let x = $x.opt_arg($e);
            #[cfg(not(feature = "opt_arg"))]
            let x = compile_error!("Requires 'opt_arg' feature");
            x
        }};
        ($x:ident; ($e:tt..)) => {
            $x.args($e)
        };
        ($x:ident; $e:tt) => {
            $x.arg($crate::internal::cmd_partial!($e))
        };
    }

    pub use arg;
    pub use cmd_partial;
}

#[macro_export]
/// Create a Command with the `cmd!` macro and call output() on it
macro_rules! run {
    ($app:tt $($q:tt)*) => {
        cmd!($app $($q)*).output()
    }
}

#[macro_export]
/// Create a Command with the `cmd!` macro and call status() on it
macro_rules! exec {
    ($app:tt $($q:tt)*) => {
        cmd!($app $($q)*).status()
    }
}

#[macro_export]
/// Create a Command with the `cmd!` macro and call spawn() on it
macro_rules! spawn {
    ($app:tt $($q:tt)*) => {
        cmd!($app $($q)*).spawn()
    }
}

#[macro_export]
/// Create an iterator to add args to an existing Command
/// ```
/// # use std::process::Command;
/// # use cmd_macro::args;
/// let mut x = Command::new("echo");
/// x.args(args!(some text here));
/// ```
//TODO: Can probably be more sophisticated
macro_rules! args {
    ($name:tt $($t:tt)*) => {

        [std::ffi::OsStr::new($crate::internal::cmd_partial!($name))]
            .into_iter()
            .chain($crate::cmd!("" $($t)*).get_args())
    }

}

#[macro_export]
/// Create a std::process::Output in the style of a terminal line
///
/// The first item is the program name. Following items are passed as args
///
/// Example
/// ```
/// # use cmd_macro::cmd;
/// cmd!(echo Hello World);
/// ```
/// Single words are stringified
/// ```no_test
/// // (You can not actually compare Commands)
/// cmd!(echo test) == cmd!(echo "test")
/// ```
/// Escaping spaces with quotes is possible
/// ```
/// # use cmd_macro::cmd;
/// let mut x = cmd!(echo "Hello World!");
/// ```
/// Identifiers in parantheses are interpolated
/// ```no_run
/// # use cmd_macro::cmd;
/// let name = "Steve";
/// cmd!(echo (name));
/// ```
/// Identifiers followed by `..` are interpolated as iterators
/// ```no_run
/// # use cmd_macro::cmd;
/// let names = ["Steve", "Mike"];
/// cmd!(echo (names..));
/// ```
/// Use `var(name)` to interpolate env vars
/// ```no_run
/// # use cmd_macro::cmd;
/// cmd!(echo (var(PATH)));
/// ```
/// Note that this will use the callers environment variables,
/// not any passed into the command
macro_rules! cmd {
    ($app:tt $($q:tt)*) => {
        {
            let mut x = std::process::Command::new($crate::internal::cmd_partial!($app));
            $($crate::internal::arg!(x; $q);)*
            x
        }
    };
}

#[cfg(test)]
mod test {

    use std::env;
    use std::process::Command;

    use crate::exec;
    use crate::output::OutputExt;
    use crate::run;

    #[test]
    fn hello_world() {
        exec!(echo "Hello World!").unwrap();
    }

    #[test]
    fn path() {
        env::set_var("MY_VAR", "my_value");
        assert_eq!("my_value", run!(echo(var(MY_VAR))).unwrap().stdout_lossy());
    }

    #[test]
    fn hello_world_array() {
        let worlds = ["overworld", "nether", "end"];
        let result = run!(echo Hello (worlds..)).unwrap();
        assert_eq!("Hello overworld nether end", result.stdout().unwrap())
    }

    #[test]
    fn interpolate() {
        let name = "Steve";
        let result = run!(echo Hello (name)).unwrap();
        assert_eq!(format!("Hello {name}"), result.stdout().unwrap());
    }

    #[test]
    fn interpolate_option() {
        use crate::opt_arg::OptionalArgExtension;
        let name = Some("Steve");
        let result = run!(echo Hello (name?)).unwrap();
        assert_eq!(format!("Hello Steve"), result.stdout().unwrap());
    }

    #[test]
    fn interpolate_option_flag_some() {
        use crate::opt_arg::OptionalArgExtension;
        let name = Some("Steve");
        let result = run!(echo Hello ("Lord" name ?)).unwrap();
        assert_eq!(format!("Hello Lord Steve"), result.stdout().unwrap());
    }

    #[test]
    fn interpolate_option_flag_none() {
        use crate::opt_arg::OptionalArgExtension;
        let name: Option<String> = None;
        let result = run!(echo Hello ("Lord" name ?)).unwrap();
        assert_eq!("Hello", result.stdout().unwrap());
    }

    #[test]
    fn args_macro() {
        let mut x = Command::new("echo");
        let name = "Steve";
        x.args(args!(Hello(name)));
        let result = x.output().unwrap();
        assert_eq!(format!("Hello Steve"), result.stdout().unwrap());
    }

    #[test]
    fn args_macro_optional() {
        use crate::opt_arg::OptionalArgExtension;
        let mut x = Command::new("echo");
        let name = "Steve";
        let other = Some("Paul");
        x.args(args!(Hello (name) (and other ?)));
        let result = x.output().unwrap();
        assert_eq!(format!("Hello Steve and Paul"), result.stdout().unwrap());
    }
}
