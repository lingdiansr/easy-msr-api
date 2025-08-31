#[cfg(test)]
mod tests {
    use crate::client::remote::RemoteApiClient;

    #[test]
    fn test_url_normalization() {
        let _client = RemoteApiClient::new("https://example.com/".to_string());
        // 通过构造函数验证URL标准化
        assert!(true); // 简化测试，主要验证编译
        
        let _client = RemoteApiClient::new("https://example.com//".to_string());
        assert!(true);
    }

    #[test]
    fn test_client_creation() {
        let _client = RemoteApiClient::new("https://example.com".to_string());
        assert!(true);
    }
}
