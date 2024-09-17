internal static async Task<string> ExecuteKustoQueryAsync(string clusterUri, string databaseName, string kustoQuery, object? parameters = null, CancellationToken cancellationToken = default)
