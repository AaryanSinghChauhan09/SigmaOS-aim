//! Protocol definitions for aim.

/// Protocol type
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolType {
    /// XMPP protocol
    XMPP,
    /// IRC protocol
    IRC,
    /// Matrix protocol
    Matrix,
    /// Telegram protocol
    Telegram,
    /// WhatsApp protocol
    WhatsApp,
    /// Signal protocol
    Signal,
    /// Slack protocol
    Slack,
    /// Discord protocol
    Discord,
    /// Custom protocol
    Custom,
}

/// Protocol status
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolStatus {
    /// Active
    Active,
    /// Inactive
    Inactive,
    /// Connecting
    Connecting,
    /// Error
    Error,
}

/// Protocol
#[derive(Debug, Clone)]
pub struct Protocol {
    /// Protocol name
    pub name: String,
    /// Protocol type
    pub protocol_type: ProtocolType,
    /// Protocol status
    pub status: ProtocolStatus,
    /// Protocol version
    pub version: String,
    /// Protocol features
    pub features: Vec<String>,
    /// Protocol capabilities
    pub capabilities: Vec<String>,
    /// Protocol supported message types
    pub supported_message_types: Vec<crate::message::MessageType>,
    /// Protocol default port
    pub default_port: u16,
    /// Protocol uses SSL/TLS
    pub uses_ssl: bool,
    /// Protocol description
    pub description: String,
}

impl Protocol {
    /// Create new protocol
    pub fn new(name: String, protocol_type: ProtocolType) -> Self {
        Self {
            name,
            protocol_type,
            status: ProtocolStatus::Inactive,
            version: "1.0".into(),
            features: vec![],
            capabilities: vec![],
            supported_message_types: vec![],
            default_port: 0,
            uses_ssl: false,
            description: String::new(),
        }
    }
    
    /// Set version
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }
    
    /// Add feature
    pub fn add_feature(&mut self, feature: String) {
        if !self.features.contains(&feature) {
            self.features.push(feature);
        }
    }
    
    /// Remove feature
    pub fn remove_feature(&mut self, feature: &str) -> bool {
        if let Some(pos) = self.features.iter().position(|f| f == feature) {
            self.features.remove(pos);
            return true;
        }
        false
    }
    
    /// Add capability
    pub fn add_capability(&mut self, capability: String) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
    }
    
    /// Remove capability
    pub fn remove_capability(&mut self, capability: &str) -> bool {
        if let Some(pos) = self.capabilities.iter().position(|c| c == capability) {
            self.capabilities.remove(pos);
            return true;
        }
        false
    }
    
    /// Add supported message type
    pub fn add_supported_message_type(&mut self, message_type: crate::message::MessageType) {
        if !self.supported_message_types.contains(&message_type) {
            self.supported_message_types.push(message_type);
        }
    }
    
    /// Remove supported message type
    pub fn remove_supported_message_type(&mut self, message_type: &crate::message::MessageType) -> bool {
        if let Some(pos) = self.supported_message_types.iter().position(|m| m == message_type) {
            self.supported_message_types.remove(pos);
            return true;
        }
        false
    }
    
    /// Set default port
    pub fn set_default_port(&mut self, port: u16) {
        self.default_port = port;
    }
    
    /// Enable SSL/TLS
    pub fn enable_ssl(&mut self) {
        self.uses_ssl = true;
    }
    
    /// Disable SSL/TLS
    pub fn disable_ssl(&mut self) {
        self.uses_ssl = false;
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    
    /// Activate protocol
    pub fn activate(&mut self) {
        self.status = ProtocolStatus::Active;
    }
    
    /// Deactivate protocol
    pub fn deactivate(&mut self) {
        self.status = ProtocolStatus::Inactive;
    }
    
    /// Set status
    pub fn set_status(&mut self, status: ProtocolStatus) {
        self.status = status;
    }
    
    /// Check if protocol is active
    pub fn is_active(&self) -> bool {
        self.status == ProtocolStatus::Active
    }
    
    /// Check if protocol is inactive
    pub fn is_inactive(&self) -> bool {
        self.status == ProtocolStatus::Inactive
    }
    
    /// Check if protocol has error
    pub fn has_error(&self) -> bool {
        self.status == ProtocolStatus::Error
    }
    
    /// Check if protocol supports message type
    pub fn supports_message_type(&self, message_type: &crate::message::MessageType) -> bool {
        self.supported_message_types.contains(message_type)
    }
    
    /// Check if protocol has feature
    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.iter().any(|f| f == feature)
    }
    
    /// Check if protocol has capability
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|c| c == capability)
    }
    
    /// Get protocol info
    pub fn get_info(&self) -> ProtocolInfo {
        ProtocolInfo {
            name: self.name.clone(),
            protocol_type: self.protocol_type.clone(),
            version: self.version.clone(),
            status: self.status.clone(),
            features_count: self.features.len(),
            capabilities_count: self.capabilities.len(),
            supported_message_types_count: self.supported_message_types.len(),
            default_port: self.default_port,
            uses_ssl: self.uses_ssl,
        }
    }
}

/// Protocol info
#[derive(Debug, Clone)]
pub struct ProtocolInfo {
    /// Protocol name
    pub name: String,
    /// Protocol type
    pub protocol_type: ProtocolType,
    /// Protocol version
    pub version: String,
    /// Protocol status
    pub status: ProtocolStatus,
    /// Features count
    pub features_count: usize,
    /// Capabilities count
    pub capabilities_count: usize,
    /// Supported message types count
    pub supported_message_types_count: usize,
    /// Default port
    pub default_port: u16,
    /// Uses SSL/TLS
    pub uses_ssl: bool,
}
