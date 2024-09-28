pub trait CommandExt {
    fn run(&mut self) -> std::io::Result<()>;

}

pub trait OutputExt {
    fn stdout(&self) -> &str;
    fn stderr(&self) -> &str;
}

impl CommandExt for std::process::Command {
    fn run(&mut self) -> std::io::Result<()> {
        let status = self.status()?;
        if !status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Command failed with status: {}", status),
            ));
        }
        Ok(())
    }
}

impl OutputExt for std::process::Output {
    fn stdout(&self) -> &str {
        std::str::from_utf8(&self.stdout).expect("Failed to convert stdout to str")
    }

    fn stderr(&self) -> &str {
        std::str::from_utf8(&self.stderr).expect("Failed to convert stderr to str")
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
