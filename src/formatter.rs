use std::fs;
use std::path::Path;
use std::io::{BufReader, BufWriter, Write};
use std::fs::File;

pub struct Formatter {
    line_length: usize,
}

impl Formatter {
    pub fn new(line_length: usize) -> Self {
        Self { line_length }
    }

    pub fn format_file(&self, file_path: &Path) -> Result<(), std::io::Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut writer = BufWriter::new(File::create(file_path)?);

        for line in reader.lines() {
            let formatted_line = self.format_line(&line?);
            writeln!(writer, "{}", formatted_line)?;
        }
        writer.flush()?;
        Ok(())
    }

    fn format_content(&self, content: &str) -> String {
        let mut formatted_lines = Vec::new();
        for line in content.lines() {
            if line.len() > self.line_length {
                formatted_lines.push(self.wrap_line(line));
            } else {
                formatted_lines.push(line.to_string());
            }
        }
        formatted_lines.join("\n")
    }

    fn wrap_line(&self, line: &str) -> String {
        let mut wrapped_line = String::new();
        let mut current_length = 0;
        for word in line.split_whitespace() {
            if current_length + word.len() + 1 > self.line_length {
                wrapped_line.push('\n');
                current_length = 0;
            }
            if current_length > 0 {
                wrapped_line.push(' ');
                current_length += 1;
            }
            wrapped_line.push_str(word);
            current_length += word.len();
        }
        wrapped_line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_line() {
        let formatter = Formatter::new(40);
        let long_line = "internal static async Task<string> ExecuteKustoQueryAsync(string clusterUri, string databaseName, string kustoQuery, object? parameters = null, CancellationToken cancellationToken = default)";
        let expected = "internal static async Task<string> ExecuteKustoQueryAsync(\nstring clusterUri,\ndatabaseName,\nkustoQuery,\nobject? parameters = null,\nCancellationToken cancellationToken = default)";
        assert_eq!(formatter.wrap_line(long_line), expected);
    }

    #[test]
    fn test_format_content() {
        let formatter = Formatter::new(40);
        let content = "internal static async Task<string> ExecuteKustoQueryAsync(string clusterUri, string databaseName, string kustoQuery, object? parameters = null, CancellationToken cancellationToken = default)\nshort line";
        let expected = "internal static async Task<string> ExecuteKustoQueryAsync(\nstring clusterUri,\ndatabaseName,\nkustoQuery,\nobject? parameters = null,\nCancellationToken cancellationToken = default)\nshort line";
        assert_eq!(formatter.format_content(content), expected);
    }
}
