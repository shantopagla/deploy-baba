//! Network configuration types.
//!
//! Defines networking components including VPCs, subnets, security groups,
//! and traffic rules.

use serde::{Deserialize, Serialize};

/// Network configuration.
///
/// Defines VPC settings, subnets, and security policies.
///
/// # Example
///
/// ```
/// use infra_types::{NetworkConfig, Subnet, SecurityGroup};
///
/// let network = NetworkConfig::new("10.0.0.0/16");
/// assert_eq!(network.vpc_cidr, "10.0.0.0/16");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// VPC CIDR block (e.g., "10.0.0.0/16")
    pub vpc_cidr: String,

    /// Subnets within this network
    #[serde(default)]
    pub subnets: Vec<Subnet>,

    /// Security groups for traffic control
    #[serde(default)]
    pub security_groups: Vec<SecurityGroup>,
}

impl NetworkConfig {
    /// Create a new network configuration.
    pub fn new(vpc_cidr: impl Into<String>) -> Self {
        Self {
            vpc_cidr: vpc_cidr.into(),
            subnets: Vec::new(),
            security_groups: Vec::new(),
        }
    }

    /// Add a subnet to this network.
    pub fn add_subnet(mut self, subnet: Subnet) -> Self {
        self.subnets.push(subnet);
        self
    }

    /// Add a security group to this network.
    pub fn add_security_group(mut self, sg: SecurityGroup) -> Self {
        self.security_groups.push(sg);
        self
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            vpc_cidr: "10.0.0.0/16".to_string(),
            subnets: Vec::new(),
            security_groups: Vec::new(),
        }
    }
}

/// Subnet configuration within a network.
///
/// # Example
///
/// ```
/// use infra_types::Subnet;
///
/// let subnet = Subnet::new("10.0.1.0/24", "us-east-1a");
/// assert_eq!(subnet.cidr, "10.0.1.0/24");
/// assert!(!subnet.is_public);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subnet {
    /// CIDR block for this subnet (e.g., "10.0.1.0/24")
    pub cidr: String,

    /// Availability zone or location
    pub availability_zone: String,

    /// Whether this is a public subnet (has internet access)
    #[serde(default)]
    pub is_public: bool,
}

impl Subnet {
    /// Create a new subnet.
    pub fn new(
        cidr: impl Into<String>,
        availability_zone: impl Into<String>,
    ) -> Self {
        Self {
            cidr: cidr.into(),
            availability_zone: availability_zone.into(),
            is_public: false,
        }
    }

    /// Mark this subnet as public.
    pub fn as_public(mut self) -> Self {
        self.is_public = true;
        self
    }
}

/// Security group for traffic control.
///
/// Defines inbound and outbound traffic rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityGroup {
    /// Security group name
    pub name: String,

    /// Description of the security group's purpose
    pub description: String,

    /// Inbound traffic rules
    #[serde(default)]
    pub ingress: Vec<IngressRule>,

    /// Outbound traffic rules
    #[serde(default)]
    pub egress: Vec<EgressRule>,
}

impl SecurityGroup {
    /// Create a new security group.
    ///
    /// # Example
    ///
    /// ```
    /// use infra_types::SecurityGroup;
    ///
    /// let sg = SecurityGroup::new("app-sg", "Application security group");
    /// assert_eq!(sg.name, "app-sg");
    /// assert!(sg.ingress.is_empty());
    /// ```
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            ingress: Vec::new(),
            egress: Vec::new(),
        }
    }

    /// Add an ingress rule to this security group.
    pub fn add_ingress(mut self, rule: IngressRule) -> Self {
        self.ingress.push(rule);
        self
    }

    /// Add an egress rule to this security group.
    pub fn add_egress(mut self, rule: EgressRule) -> Self {
        self.egress.push(rule);
        self
    }
}

/// Inbound traffic rule.
///
/// # Example
///
/// ```
/// use infra_types::IngressRule;
///
/// let rule = IngressRule::new("tcp", 443, 443, "0.0.0.0/0");
/// assert_eq!(rule.port_range, "443");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngressRule {
    /// Protocol: tcp, udp, icmp, or -1 (all)
    pub protocol: String,

    /// Port or port range (e.g., "80", "80-443")
    pub port_range: String,

    /// CIDR block or security group ID (e.g., "0.0.0.0/0" or "sg-xxx")
    pub source: String,
}

impl IngressRule {
    /// Create a new ingress rule.
    pub fn new(
        protocol: impl Into<String>,
        from_port: u16,
        to_port: u16,
        source: impl Into<String>,
    ) -> Self {
        let port_range = if from_port == to_port {
            from_port.to_string()
        } else {
            format!("{}-{}", from_port, to_port)
        };

        Self {
            protocol: protocol.into(),
            port_range,
            source: source.into(),
        }
    }
}

/// Outbound traffic rule.
///
/// # Example
///
/// ```
/// use infra_types::EgressRule;
///
/// let rule = EgressRule::new("tcp", 443, 443, "0.0.0.0/0");
/// assert_eq!(rule.port_range, "443");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EgressRule {
    /// Protocol: tcp, udp, icmp, or -1 (all)
    pub protocol: String,

    /// Port or port range (e.g., "80", "80-443")
    pub port_range: String,

    /// Destination CIDR block or security group ID
    pub destination: String,
}

impl EgressRule {
    /// Create a new egress rule.
    pub fn new(
        protocol: impl Into<String>,
        from_port: u16,
        to_port: u16,
        destination: impl Into<String>,
    ) -> Self {
        let port_range = if from_port == to_port {
            from_port.to_string()
        } else {
            format!("{}-{}", from_port, to_port)
        };

        Self {
            protocol: protocol.into(),
            port_range,
            destination: destination.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config_creation() {
        let network = NetworkConfig::new("10.0.0.0/16");
        assert_eq!(network.vpc_cidr, "10.0.0.0/16");
        assert!(network.subnets.is_empty());
    }

    #[test]
    fn test_network_config_add_subnet() {
        let subnet = Subnet::new("10.0.1.0/24", "us-east-1a");
        let network = NetworkConfig::new("10.0.0.0/16").add_subnet(subnet);

        assert_eq!(network.subnets.len(), 1);
        assert_eq!(network.subnets[0].cidr, "10.0.1.0/24");
    }

    #[test]
    fn test_subnet_as_public() {
        let subnet = Subnet::new("10.0.1.0/24", "us-east-1a").as_public();
        assert!(subnet.is_public);
    }

    #[test]
    fn test_security_group_creation() {
        let sg = SecurityGroup::new("app-sg", "Application security group");
        assert_eq!(sg.name, "app-sg");
        assert_eq!(sg.description, "Application security group");
        assert!(sg.ingress.is_empty());
        assert!(sg.egress.is_empty());
    }

    #[test]
    fn test_security_group_add_rules() {
        let ingress = IngressRule::new("tcp", 443, 443, "0.0.0.0/0");
        let egress = EgressRule::new("tcp", 443, 443, "0.0.0.0/0");
        let sg = SecurityGroup::new("app-sg", "Test").add_ingress(ingress).add_egress(egress);

        assert_eq!(sg.ingress.len(), 1);
        assert_eq!(sg.egress.len(), 1);
    }

    #[test]
    fn test_ingress_rule_port_range_single() {
        let rule = IngressRule::new("tcp", 443, 443, "0.0.0.0/0");
        assert_eq!(rule.port_range, "443");
    }

    #[test]
    fn test_ingress_rule_port_range_multiple() {
        let rule = IngressRule::new("tcp", 80, 443, "0.0.0.0/0");
        assert_eq!(rule.port_range, "80-443");
    }

    #[test]
    fn test_egress_rule_port_range() {
        let rule = EgressRule::new("tcp", 443, 443, "0.0.0.0/0");
        assert_eq!(rule.port_range, "443");
    }
}
