use std::{ffi::OsStr, process::Command};

#[cfg(test)]
mod test {
    use std::process::Command;

    use crate::opt_arg::OptionalArgExtension as _;

    #[test]
    fn opt_arg_some() {
        let mut cmd = Command::new("echo");
        cmd.opt_arg(Some("test"));
        assert_eq!(cmd.get_args().into_iter().collect::<Vec<_>>(), ["test"]);
    }
    #[test]
    fn opt_arg_none() {
        let mut cmd = Command::new("echo");
        cmd.arg("test").opt_arg(Option::<&str>::None);
        assert_eq!(cmd.get_args().into_iter().collect::<Vec<_>>(), ["test"]);
    }
    #[test]
    fn opt_arg_flag_some() {
        let mut cmd = Command::new("echo");
        cmd.opt_arg(("foo", Some("bar")));
        assert_eq!(
            cmd.get_args().into_iter().collect::<Vec<_>>(),
            ["foo", "bar"]
        );
    }
    #[test]
    fn opt_arg_flag_none() {
        let mut cmd = Command::new("echo");
        cmd.arg("test").opt_arg(("foo", Option::<&str>::None));
        assert_eq!(cmd.get_args().into_iter().collect::<Vec<_>>(), ["test"]);
    }
}

pub trait OptionalArgExtension<T> {
    /// If T is an Option<Val>, only include this argument if it is `Some(Val)`
    /// 
    /// If T is a (Name, Option<Val>) the Name will also only be included if the Option is Some(Val)
    /// 
    /// ## Examples
    /// 
    /// ```
    /// # use std::process::Command;
    /// # use cmd_macro::opt_arg::OptionalArgExtension;
    /// let maybe = Some("test");
    /// Command::new("echo").opt_arg(maybe).status();
    /// ```
    /// 
    /// Only include the "-c" if there is a command to run
    /// ```
    /// # use std::process::Command;
    /// # use cmd_macro::opt_arg::OptionalArgExtension;
    /// let maybe_flag = ("-c", Some("emacs"));
    /// Command::new("bash").opt_arg(maybe_flag).status();
    /// ```
    fn opt_arg(&mut self, val: T) -> &mut Self;
}

impl<Value: AsRef<OsStr>> OptionalArgExtension<Option<Value>> for Command {
    fn opt_arg(&mut self, val: Option<Value>) -> &mut Self {
        if let Some(value) = val {
            self.arg(value);
        }
        self
    }
}

impl<Name: AsRef<OsStr>, Value: AsRef<OsStr>> OptionalArgExtension<(Name, Option<Value>)>
    for Command
{
    fn opt_arg(&mut self, val: (Name, Option<Value>)) -> &mut Self {
        if let Some(value) = val.1 {
            self.arg(val.0.as_ref()).arg(value.as_ref());
        }
        self
    }
}
