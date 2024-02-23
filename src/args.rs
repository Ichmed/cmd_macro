use std::ffi::OsStr;

use crate::opt_arg::OptionalArgExtension;

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
    ($app:tt $($q:tt)*) => {
        {
            let mut x = $crate::args::ArgContainer::new($crate::internal::cmd_partial!($app));
            $($crate::internal::arg!(x; $q);)*
            x.args.into_iter()
        }
    };
}

#[doc(hidden)]
pub struct ArgContainer<'a> {
    pub args: Vec<&'a OsStr>,
}

impl<'a> ArgContainer<'a> {
    pub fn new<T: AsRef<OsStr> + ?Sized>(arg: &'a T) -> Self {
        Self {
            args: vec![arg.as_ref()],
        }
    }

    pub fn arg<T: AsRef<OsStr> + ?Sized>(&mut self, arg: &'a T) -> &mut Self {
        self.args.push(arg.as_ref());
        self
    }

    pub fn args<T: AsRef<OsStr> + ?Sized + 'a>(
        &mut self,
        args: impl IntoIterator<Item = &'a T>,
    ) -> &mut Self {
        self.args.extend(args.into_iter().map(|arg| arg.as_ref()));
        self
    }
}

impl<'a, T: AsRef<OsStr> + ?Sized> OptionalArgExtension<Option<&'a T>> for ArgContainer<'a> {
    fn opt_arg(&mut self, val: Option<&'a T>) -> &mut Self {
        if let Some(arg) = val {
            self.arg(arg);
        }
        self
    }
}

impl<'a, Name, Value, I> OptionalArgExtension<(&'a Name, I)> for ArgContainer<'a>
where
    Name: AsRef<OsStr> + ?Sized + 'a,
    Value: AsRef<OsStr> + ?Sized + 'a,
    I: IntoIterator<Item = &'a Value>,
{
    fn opt_arg(&mut self, val: (&'a Name, I)) -> &mut Self {
        let (flag, i) = val;
        let mut i = i.into_iter();
        if let Some(value) = i.next() {
            self.arg(flag.as_ref()).arg(value.as_ref());
            self.args(i.map(|x| x.as_ref()));
        }
        self
    }
}

#[cfg(test)]
mod test {
    use std::process::Command;

    use crate::opt_arg::OptionalArgExtension as _;
    use crate::output::OutputExt as _;

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
        let mut x = Command::new("echo");
        let name = "Steve";
        let other = Some("Paul");
        x.args(args!(Hello (name) (and other ?)));
        let result = x.output().unwrap();
        assert_eq!(format!("Hello Steve and Paul"), result.stdout().unwrap());
    }
}
