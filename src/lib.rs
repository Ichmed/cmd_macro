use std::{process::Output, string::FromUtf8Error};



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
/// Create a std::process::Output in the style of a terminal line
/// 
/// The first item is the program name. Following items are passed as args
/// 
/// Example
/// ```no_run
/// cmd!(echo Hello World)
/// ```
/// Single words are stringified 
/// ```no_run
/// cmd!(echo test) == cmd!(echo "test")
/// ```
/// Escaping spaces with quotes is possible 
/// ```
/// cmd!(echo "Hello World!")
/// ```
/// Identifiers in parantheses are interpolated
/// ```no_run
/// let name = "Steve";
/// cmd!(echo (name))
/// ```
/// Identifiers followed by `..` are interpolated as iterators
/// ```no_run
/// let names = ["Steve", "Mike"];
/// cmd!(echo (names..))
/// ```
/// SUse `var(name)` to interpolate env vars
/// ```no_run
/// cmd!(echo (var(PATH)))
/// ```
macro_rules! cmd {
    ($app:tt $($q:tt)*) => {
        {
            let mut x = std::process::Command::new($crate::internal::cmd_partial!($app));
            $($crate::internal::arg!(x; $q);)*
            x
        }
    };
}

pub trait OutputExt {
    /// Tries to parse stdout into a valid [String] without trailing newlines
    fn stdout(&self) -> Result<String, FromUtf8Error>;
    /// Tries to parse stderr into a valid [String] without trailing newlines
    fn stderr(&self) -> Result<String, FromUtf8Error>;
    /// Parses stdout lossily into a valid [String] without trailing newlines
    fn stdout_lossy(&self) -> String;
    /// Parses stderr lossily into a valid [String] without trailing newlines
    fn stderr_lossy(&self) -> String;
}

impl OutputExt for Output {
    fn stdout(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.stdout.clone()).map(|x| x.trim_end_matches('\n').to_owned())
    }
    
    fn stderr(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.stderr.clone()).map(|x| x.trim_end_matches('\n').to_owned())
    }

    fn stdout_lossy(&self) -> String {
        String::from_utf8_lossy(&self.stdout).trim_end_matches('\n').to_owned()
    }
    
    fn stderr_lossy(&self) -> String {
        String::from_utf8_lossy(&self.stderr).trim_end_matches('\n').to_owned()
    }
}



#[cfg(test)]
mod test {

    use crate::OutputExt;
    use crate::exec;
    use crate::run;

    #[test]
    fn hello_world() {
        exec!(echo "Hello World!").unwrap();
    }

    #[test]
    fn path() {
        exec!(echo (var(PATH))).unwrap();
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
}