//! RFC 1918 private network validation utilities

use crate::model::{VlanError, VlanResult};
use ipnetwork::Ipv4Network;
use std::net::Ipv4Addr;

/// RFC 1918 private address ranges
pub struct Rfc1918Ranges {
    /// Class A: 10.0.0.0/8 (10.0.0.0 to 10.255.255.255)
    pub class_a: (Ipv4Addr, Ipv4Addr),
    /// Class B: 172.16.0.0/12 (172.16.0.0 to 172.31.255.255)  
    pub class_b: (Ipv4Addr, Ipv4Addr),
    /// Class C: 192.168.0.0/16 (192.168.0.0 to 192.168.255.255)
    pub class_c: (Ipv4Addr, Ipv4Addr),
}

impl Default for Rfc1918Ranges {
    fn default() -> Self {
        Self {
            class_a: (Ipv4Addr::new(10, 0, 0, 0), Ipv4Addr::new(10, 255, 255, 255)),
            class_b: (
                Ipv4Addr::new(172, 16, 0, 0),
                Ipv4Addr::new(172, 31, 255, 255),
            ),
            class_c: (
                Ipv4Addr::new(192, 168, 0, 0),
                Ipv4Addr::new(192, 168, 255, 255),
            ),
        }
    }
}

/// Check if an IPv4 address is within RFC 1918 private address space
pub fn is_rfc1918_addr(addr: Ipv4Addr) -> bool {
    let ranges = Rfc1918Ranges::default();

    // Check Class A (10.0.0.0/8)
    if addr >= ranges.class_a.0 && addr <= ranges.class_a.1 {
        return true;
    }

    // Check Class B (172.16.0.0/12)
    if addr >= ranges.class_b.0 && addr <= ranges.class_b.1 {
        return true;
    }

    // Check Class C (192.168.0.0/16)
    if addr >= ranges.class_c.0 && addr <= ranges.class_c.1 {
        return true;
    }

    false
}

/// Check if an IPv4 network is entirely within RFC 1918 private address space
pub fn is_rfc1918_network(network: &Ipv4Network) -> bool {
    let network_addr = network.network();
    let broadcast_addr = network.broadcast();

    // Both network and broadcast addresses must be in RFC 1918 space
    is_rfc1918_addr(network_addr) && is_rfc1918_addr(broadcast_addr)
}

/// Validate that a network string represents a valid RFC 1918 network
pub fn validate_rfc1918_network_string(network_str: &str) -> VlanResult<Ipv4Network> {
    // Parse the network string
    let network = network_str
        .parse::<Ipv4Network>()
        .map_err(|e| VlanError::network_parsing(format!("Failed to parse '{network_str}': {e}")))?;

    // Validate RFC 1918 compliance
    if !is_rfc1918_network(&network) {
        return Err(VlanError::NonRfc1918Network(network_str.to_string()));
    }

    Ok(network)
}

/// Convert a "10.x.x.x" format string to a proper RFC 1918 network
pub fn convert_x_format_to_network(x_format: &str) -> VlanResult<Ipv4Network> {
    if let Some(base) = x_format.strip_suffix(".x") {
        let network_str = format!("{base}.0/24");
        validate_rfc1918_network_string(&network_str)
    } else {
        Err(VlanError::network_parsing(format!(
            "Invalid x format: {x_format}"
        )))
    }
}

/// Generate a random RFC 1918 Class A network (10.x.y.0/24)
pub fn generate_random_class_a_network<R: rand::Rng>(rng: &mut R) -> Ipv4Network {
    let second_octet = rng.random_range(1..=254);
    let third_octet = rng.random_range(1..=254);

    // This is guaranteed to be RFC 1918 compliant
    format!("10.{second_octet}.{third_octet}.0/24")
        .parse()
        .expect("Generated network should be valid")
}

/// Generate a random RFC 1918 Class B network (172.16-31.x.0/24)
pub fn generate_random_class_b_network<R: rand::Rng>(rng: &mut R) -> Ipv4Network {
    let second_octet = rng.random_range(16..=31);
    let third_octet = rng.random_range(1..=254);

    // This is guaranteed to be RFC 1918 compliant
    format!("172.{second_octet}.{third_octet}.0/24")
        .parse()
        .expect("Generated network should be valid")
}

/// Generate a random RFC 1918 Class C network (192.168.x.0/24)
pub fn generate_random_class_c_network<R: rand::Rng>(rng: &mut R) -> Ipv4Network {
    let third_octet = rng.random_range(1..=254);

    // This is guaranteed to be RFC 1918 compliant
    format!("192.168.{third_octet}.0/24")
        .parse()
        .expect("Generated network should be valid")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_rfc1918_addr_validation() {
        // Valid RFC 1918 addresses
        assert!(is_rfc1918_addr(Ipv4Addr::new(10, 0, 0, 1)));
        assert!(is_rfc1918_addr(Ipv4Addr::new(10, 255, 255, 254)));
        assert!(is_rfc1918_addr(Ipv4Addr::new(172, 16, 0, 1)));
        assert!(is_rfc1918_addr(Ipv4Addr::new(172, 31, 255, 254)));
        assert!(is_rfc1918_addr(Ipv4Addr::new(192, 168, 0, 1)));
        assert!(is_rfc1918_addr(Ipv4Addr::new(192, 168, 255, 254)));

        // Invalid RFC 1918 addresses (public addresses)
        assert!(!is_rfc1918_addr(Ipv4Addr::new(8, 8, 8, 8)));
        assert!(!is_rfc1918_addr(Ipv4Addr::new(1, 1, 1, 1)));
        assert!(!is_rfc1918_addr(Ipv4Addr::new(172, 15, 0, 1))); // Just outside Class B
        assert!(!is_rfc1918_addr(Ipv4Addr::new(172, 32, 0, 1))); // Just outside Class B
    }

    #[test]
    fn test_rfc1918_network_validation() {
        // Valid RFC 1918 networks
        let valid_networks = vec![
            "10.1.2.0/24",
            "172.16.0.0/24",
            "192.168.1.0/24",
            "10.0.0.0/8",     // Entire Class A
            "172.16.0.0/12",  // Entire Class B range
            "192.168.0.0/16", // Entire Class C range
        ];

        for network_str in valid_networks {
            let network = network_str.parse::<Ipv4Network>().unwrap();
            assert!(
                is_rfc1918_network(&network),
                "Network {network_str} should be RFC 1918"
            );
        }

        // Invalid RFC 1918 networks (public networks)
        let invalid_networks = vec![
            "8.8.8.0/24",    // Google DNS
            "1.1.1.0/24",    // Cloudflare DNS
            "172.15.0.0/24", // Just outside Class B
            "172.32.0.0/24", // Just outside Class B
        ];

        for network_str in invalid_networks {
            let network = network_str.parse::<Ipv4Network>().unwrap();
            assert!(
                !is_rfc1918_network(&network),
                "Network {network_str} should not be RFC 1918"
            );
        }
    }

    #[test]
    fn test_validate_rfc1918_network_string() {
        // Valid cases
        assert!(validate_rfc1918_network_string("10.1.2.0/24").is_ok());
        assert!(validate_rfc1918_network_string("172.16.0.0/24").is_ok());
        assert!(validate_rfc1918_network_string("192.168.1.0/24").is_ok());

        // Invalid cases - non-RFC 1918
        assert!(validate_rfc1918_network_string("8.8.8.0/24").is_err());

        // Invalid cases - malformed
        assert!(validate_rfc1918_network_string("invalid").is_err());
        assert!(validate_rfc1918_network_string("10.1.2.256/24").is_err()); // Invalid octet
    }

    #[test]
    fn test_convert_x_format_to_network() {
        // Valid conversions
        let result = convert_x_format_to_network("10.1.2.x").unwrap();
        assert_eq!(result.to_string(), "10.1.2.0/24");

        let result = convert_x_format_to_network("192.168.50.x").unwrap();
        assert_eq!(result.to_string(), "192.168.50.0/24");

        // Invalid formats
        assert!(convert_x_format_to_network("10.1.2.1").is_err());
        assert!(convert_x_format_to_network("invalid").is_err());

        // Non-RFC 1918 should fail
        assert!(convert_x_format_to_network("8.8.8.x").is_err());
    }

    #[test]
    fn test_random_network_generation() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);

        // Test Class A generation
        for _ in 0..10 {
            let network = generate_random_class_a_network(&mut rng);
            assert!(is_rfc1918_network(&network));
            assert!(network.to_string().starts_with("10."));
            assert!(network.to_string().ends_with(".0/24"));
        }

        // Test Class B generation
        for _ in 0..10 {
            let network = generate_random_class_b_network(&mut rng);
            assert!(is_rfc1918_network(&network));
            assert!(network.to_string().starts_with("172."));
            assert!(network.to_string().ends_with(".0/24"));
        }

        // Test Class C generation
        for _ in 0..10 {
            let network = generate_random_class_c_network(&mut rng);
            assert!(is_rfc1918_network(&network));
            assert!(network.to_string().starts_with("192.168."));
            assert!(network.to_string().ends_with(".0/24"));
        }
    }

    #[test]
    fn test_class_ranges() {
        let ranges = Rfc1918Ranges::default();

        // Test Class A boundaries
        assert_eq!(ranges.class_a.0, Ipv4Addr::new(10, 0, 0, 0));
        assert_eq!(ranges.class_a.1, Ipv4Addr::new(10, 255, 255, 255));

        // Test Class B boundaries
        assert_eq!(ranges.class_b.0, Ipv4Addr::new(172, 16, 0, 0));
        assert_eq!(ranges.class_b.1, Ipv4Addr::new(172, 31, 255, 255));

        // Test Class C boundaries
        assert_eq!(ranges.class_c.0, Ipv4Addr::new(192, 168, 0, 0));
        assert_eq!(ranges.class_c.1, Ipv4Addr::new(192, 168, 255, 255));
    }
}
