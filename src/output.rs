use std::{process::Output, string::FromUtf8Error};

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
        String::from_utf8_lossy(&self.stdout)
            .trim_end_matches('\n')
            .to_owned()
    }

    fn stderr_lossy(&self) -> String {
        String::from_utf8_lossy(&self.stderr)
            .trim_end_matches('\n')
            .to_owned()
    }
}
