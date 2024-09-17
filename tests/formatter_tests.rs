use std::path::Path;
use super::Formatter;

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
