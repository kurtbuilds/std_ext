pub trait CommandExt {
    fn run(&mut self) -> std::io::Result<()>;

}

pub trait OutputExt {
    fn stdout(&mut self) -> String;
    fn stderr(&mut self) -> String;
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
    fn stdout(&mut self) -> String {
        let bytes = std::mem::take(&mut self.stdout);
        String::from_utf8(bytes).expect("Failed to convert stdout to string")
    }

    fn stderr(&mut self) -> String {
        let bytes = std::mem::take(&mut self.stderr);
        String::from_utf8(bytes).expect("Failed to convert stdout to string")
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
