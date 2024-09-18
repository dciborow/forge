use std::path::Path;
use crate::formatter::Formatter;

#[test]
fn test_wrap_line() {
    let formatter = Formatter::new(80);
    let long_line = "internal static async Task<string> ExecuteKustoQueryAsync(string clusterUri, string databaseName, string kustoQuery, object? parameters = null, CancellationToken cancellationToken = default)";
    let expected = "internal static async Task<string> ExecuteKustoQueryAsync(\n    string clusterUri,\n    string databaseName,\n    string kustoQuery,\n    object? parameters = null,\n    CancellationToken cancellationToken = default)";
    assert_eq!(formatter.wrap_line(long_line), expected);
}

#[test]
fn test_format_content() {
    let formatter = Formatter::new(80);
    let content = "internal static async Task<string> ExecuteKustoQueryAsync(string clusterUri, string databaseName, string kustoQuery, object? parameters = null, CancellationToken cancellationToken = default)\nshort line";
    let expected = "internal static async Task<string> ExecuteKustoQueryAsync(\n    string clusterUri,\n    string databaseName,\n    string kustoQuery,\n    object? parameters = null,\n    CancellationToken cancellationToken = default)\nshort line";
    assert_eq!(formatter.format_content(content), expected);
}

#[test]
fn test_format_file() {
    let formatter = Formatter::new(80);
    let file_path = Path::new("samples/sample.cs");
    formatter.format_file(file_path).unwrap();
    let formatted_content = std::fs::read_to_string(file_path).unwrap();
    let expected = "internal static async Task<string> ExecuteKustoQueryAsync(\n    string clusterUri,\n    string databaseName,\n    string kustoQuery,\n    object? parameters = null,\n    CancellationToken cancellationToken = default)\n";
    assert_eq!(formatted_content, expected);
}

#[test]
fn test_wrap_line_different_lengths() {
    let test_cases = vec![
        (80, "internal static async Task<string> ExecuteKustoQueryAsync(string clusterUri, string databaseName, string kustoQuery, object? parameters = null, CancellationToken cancellationToken = default)", "internal static async Task<string> ExecuteKustoQueryAsync(\n    string clusterUri,\n    string databaseName,\n    string kustoQuery,\n    object? parameters = null,\n    CancellationToken cancellationToken = default)"),
        (40, "This is a long line that should be wrapped", "This is a long line that should be\nwrapped"),
    ];
    for (line_length, input, expected) in test_cases {
        let formatter = Formatter::new(line_length);
        assert_eq!(formatter.wrap_line(input), expected);
    }
}
