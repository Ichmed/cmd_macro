use std::{
    fmt::Display,
    process::{ExitStatus, Output},
};

use thiserror::Error;

pub trait OkExt: Sized {
    type Success;
    /// Returns a `Success`-type if the [Command] was successful, returns [CommandFailed] otherwise
    /// 
    /// For `Result<T, std::io::Error>` the type of `Success` is `T`, for other types it is `Self` 
    ///
    /// Use this to early return when a call to another command failed
    /// ```no_run
    /// # use std::process::Command;
    /// # use humane_commands::ok::OkExt as _;
    /// let x = Command::new("echo").arg("test").output().cmd_ok().unwrap().stdout;
    /// assert_eq!(String::from_utf8_lossy(&x), "test")
    /// ```
    ///
    /// The returned Err will contain the full stderr of the [Command] if possible, if you do not want this, call `ok_no_msg` instead
    fn cmd_ok(self) -> Result<Self::Success, CommandFailed> {
        self.cmd_ok_no_msg()
    }
    
    /// Returns a `Success`-type if the [Command] was succesfull, returns [CommandFailed] otherwise
    /// 
    /// For `Result<T, std::io::Error>` the type of `Success` is `T`, for other types it is `Self`
    /// ```no_run
    /// # use std::process::Command;
    /// # use humane_commands::ok::OkExt;
    /// Command::new("foo").output().cmd_ok_no_msg().unwrap();
    /// ```
    ///
    /// The returned Err will not contain any output from the [Command]
    fn cmd_ok_no_msg(self) -> Result<Self::Success, CommandFailed>;
}

#[derive(Debug, Error)]
pub enum CommandFailed {
    Status {
        status: ExitStatus,
        msg: Option<String>,
    },
    IO(std::io::Error),
}

impl Display for CommandFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Status { status, msg } => {
                f.write_str("Failed with code ")?;
                status.fmt(f)?;
                if let Some(ref msg) = msg {
                    f.write_str(" ")?;
                    f.write_str(msg)?;
                }
                Ok(())
            }
            Self::IO(err) => err.fmt(f),
        }
    }
}

impl OkExt for Output {
    type Success = Self;
    fn cmd_ok(self) -> Result<Self, CommandFailed> {
        if self.status.success() {
            Ok(self)
        } else {
            Err(CommandFailed::Status {
                status: self.status,
                msg: Some(String::from_utf8_lossy(&self.stderr).to_string()),
            })
        }
    }

    fn cmd_ok_no_msg(self) -> Result<Self, CommandFailed> {
        if self.status.success() {
            Ok(self)
        } else {
            Err(CommandFailed::Status {
                status: self.status,
                msg: None,
            })
        }
    }
}

impl OkExt for ExitStatus {
    type Success = Self;
    fn cmd_ok_no_msg(self) -> Result<Self, CommandFailed> {
        if self.success() {
            Ok(self)
        } else {
            Err(CommandFailed::Status {
                status: self,
                msg: None,
            })
        }
    }
}

impl OkExt for Result<ExitStatus, std::io::Error> {
    type Success = ExitStatus;
    fn cmd_ok_no_msg(self) -> Result<Self::Success, CommandFailed> {
        self.map_err(CommandFailed::IO)?.cmd_ok()
    }
}

impl OkExt for Result<Output, std::io::Error> {
    type Success = Output;

    fn cmd_ok_no_msg(self) -> Result<Self::Success, CommandFailed> {
        self.map_err(CommandFailed::IO)?.cmd_ok_no_msg()
    }

    fn cmd_ok(self) -> Result<Self::Success, CommandFailed> {
        self.map_err(CommandFailed::IO)?.cmd_ok()
    }
}
