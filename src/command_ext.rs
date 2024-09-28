pub trait CommandExt {
    fn run(&mut self) -> std::io::Result<()>;
    fn check_output(&mut self) -> std::io::Result<std::process::Output>;
}

pub trait OutputExt {
    fn stdout(&self) -> &str;
    fn stderr(&self) -> &str;
    fn success(&self) -> bool;
}

impl CommandExt for std::process::Command {
    fn run(&mut self) -> std::io::Result<()> {
        let status = self.status()?;
        if !status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Command exited with code: {}", status.code().unwrap()),
            ));
        }
        Ok(())
    }

    fn check_output(&mut self) -> std::io::Result<std::process::Output> {
        let output = self.output()?;
        if output.status.success() {
            Ok(output)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Command exited with code: {}", output.status.code().unwrap()),
            ))
        }
    }
}

impl OutputExt for std::process::Output {
    fn stdout(&self) -> &str {
        std::str::from_utf8(&self.stdout).expect("Failed to convert stdout to str")
    }

    fn stderr(&self) -> &str {
        std::str::from_utf8(&self.stderr).expect("Failed to convert stderr to str")
    }
    fn success(&self) -> bool {
        self.status.success()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() -> std::io::Result<()> {
        let s = std::process::Command::new("echo")
            .arg("hello")
            .output()?
            .stdout();
        assert_eq!(s, "hello\n");
        Ok(())
    }

    #[test]
    fn test_run() -> std::io::Result<()> {
        std::process::Command::new("echo")
            .arg("hello")
            .run()?;
        Ok(())
    }
}
