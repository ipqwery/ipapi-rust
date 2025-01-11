//! # ipapi
//!
//! A Rust library to query IP addresses using the ipquery.io API.
//!
//! ## Features
//! - Query details for a specific IP address
//! - Bulk query multiple IP addresses
//! - Fetch your own public IP address
//!
//! ## Example Usage
//!
//! ```rust
//! use ipapi::{query_ip, query_bulk, query_own_ip};
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Query a specific IP
//!     let ip_info = query_ip("8.8.8.8").await.unwrap();
//!     println!("{:?}", ip_info);
//! }
//! ```
//!
//! ## License
//! This project is licensed under the MIT License.

use reqwest::Error;
use serde::{Deserialize, Serialize};

/// The base URL for the ipquery.io API.
const BASE_URL: &str = "https://api.ipquery.io/";

/// Represents information about an ISP (Internet Service Provider).
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ISPInfo {
    /// The Autonomous System Number (ASN) of the ISP.
    pub asn: Option<String>,
    /// The organization associated with the ISP.
    pub org: Option<String>,
    /// The name of the ISP.
    pub isp: Option<String>,
}

/// Represents information about the geographical location of an IP address.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LocationInfo {
    /// The country name.
    pub country: Option<String>,
    /// The ISO country code.
    pub country_code: Option<String>,
    /// The city name.
    pub city: Option<String>,
    /// The state or region.
    pub state: Option<String>,
    /// The postal or ZIP code.
    pub zipcode: Option<String>,
    /// The latitude of the location.
    pub latitude: Option<f64>,
    /// The longitude of the location.
    pub longitude: Option<f64>,
    /// The timezone of the location.
    pub timezone: Option<String>,
    /// The local time in the specified timezone.
    pub localtime: Option<String>,
}

/// Represents information about potential risks associated with an IP address.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct RiskInfo {
    /// Indicates if the IP is associated with a mobile network.
    pub is_mobile: Option<bool>,
    /// Indicates if the IP is using a VPN.
    pub is_vpn: Option<bool>,
    /// Indicates if the IP is part of the Tor network.
    pub is_tor: Option<bool>,
    /// Indicates if the IP is using a proxy.
    pub is_proxy: Option<bool>,
    /// Indicates if the IP is associated with a data center.
    pub is_datacenter: Option<bool>,
    /// A score indicating the risk level (0-100).
    pub risk_score: Option<u8>,
}

/// Represents the full set of information returned by the API for an IP address.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct IPInfo {
    /// The queried IP address.
    pub ip: String,
    /// Information about the ISP.
    pub isp: Option<ISPInfo>,
    /// Information about the location.
    pub location: Option<LocationInfo>,
    /// Information about the risk level.
    pub risk: Option<RiskInfo>,
}

/// Fetches the IP information for a given IP address.
///
/// # Arguments
///
/// * `ip` - A string slice representing the IP address to query.
///
/// # Example
///
/// ```rust
/// use ipapi::query_ip;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() {
///     let ip_info = query_ip("8.8.8.8").await.unwrap();
///     println!("{:?}", ip_info);
/// }
/// ```
///
/// # Errors
///
/// Returns an error if the network request fails or the response cannot be deserialized.
pub async fn query_ip(ip: &str) -> Result<IPInfo, Error> {
    let url = format!("{}{}", BASE_URL, ip);
    let response = reqwest::get(&url).await?.json::<IPInfo>().await?;
    Ok(response)
}

/// Fetches information for multiple IP addresses.
///
/// # Arguments
///
/// * `ips` - A slice of string slices representing the list of IP addresses to query.
///
/// # Example
///
/// ```rust
/// use ipapi::query_bulk;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() {
///     let ip_infos = query_bulk(&["8.8.8.8", "1.1.1.1"]).await.unwrap();
///     println!("{:?}", ip_infos);
/// }
/// ```
///
/// # Errors
///
/// Returns an error if the network request fails or the response cannot be deserialized.
pub async fn query_bulk(ips: &[&str]) -> Result<Vec<IPInfo>, Error> {
    if ips.is_empty() {
        return Ok(vec![]);
    }
    let mut ip_list = ips.join(",");
    if ips.len() == 1 {
        ip_list.push(',');
    }
    let url = format!("{}{}", BASE_URL, ip_list);
    let response = reqwest::get(&url).await?.json::<Vec<IPInfo>>().await?;
    Ok(response)
}

/// Fetches the IP address of the current machine.
///
/// # Example
///
/// ```rust
/// use ipapi::query_own_ip;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() {
///     let ip = query_own_ip().await.unwrap();
///     println!("Your IP Address: {}", ip);
/// }
/// ```
///
/// # Errors
///
/// Returns an error if the network request fails.
pub async fn query_own_ip() -> Result<String, Error> {
    let response = reqwest::get(BASE_URL).await?.text().await?;
    Ok(response)
}
