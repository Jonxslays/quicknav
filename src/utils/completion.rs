pub struct CompletionWriter {
    content: String,
}

impl CompletionWriter {
    pub fn new() -> Self {
        Self { content: String::new() }
    }

    pub fn clone_content(&self) -> String {
        self.content.clone()
    }
}

impl std::io::Write for CompletionWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut count = 0;

        for c in buf {
            self.content.push(*c as char);
            count += 1;
        }

        Ok(count)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.content = String::new();
        Ok(())
    }
}
