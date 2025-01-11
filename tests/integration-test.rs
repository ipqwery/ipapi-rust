use ipapi::{query_ip, query_bulk, query_own_ip};

#[tokio::test]
async fn test_query_ip() {
    let result = query_ip("8.8.8.8").await;
    assert!(result.is_ok());
    let ip_info = result.unwrap();
    assert_eq!(ip_info.ip, "8.8.8.8");
    assert!(ip_info.isp.is_some());
    assert!(ip_info.location.is_some());
    assert!(ip_info.risk.is_some());
}

#[tokio::test]
async fn test_query_bulk() {
    let ips = ["8.8.8.8", "1.1.1.1"];
    let result = query_bulk(&ips).await;
    assert!(result.is_ok());
    let ip_infos = result.unwrap();
    assert_eq!(ip_infos.len(), 2);
    assert_eq!(ip_infos[0].ip, "8.8.8.8");
    assert_eq!(ip_infos[1].ip, "1.1.1.1");
}

#[tokio::test]
async fn test_query_bulk_single_ip() {
    let ips = ["8.8.8.8"];
    let result = query_bulk(&ips).await;
    assert!(result.is_ok());
    let ip_infos = result.unwrap();
    assert_eq!(ip_infos.len(), 1);
    assert_eq!(ip_infos[0].ip, "8.8.8.8");
}

#[tokio::test]
async fn test_query_bulk_no_ip() {
    let ips = [];
    let result = query_bulk(&ips).await;
    assert!(result.is_ok());
    let ip_infos = result.unwrap();
    assert!(ip_infos.is_empty());
}

#[tokio::test]
async fn test_query_own_ip() {
    let result = query_own_ip().await;
    assert!(result.is_ok());
    let ip = result.unwrap();
    assert!(!ip.is_empty());
}
