//! Account definitions for aim.

/// Account type
#[derive(Debug, Clone, PartialEq)]
pub enum AccountType {
    /// XMPP account
    XMPP,
    /// IRC account
    IRC,
    /// Matrix account
    Matrix,
    /// Telegram account
    Telegram,
    /// WhatsApp account
    WhatsApp,
    /// Signal account
    Signal,
    /// Slack account
    Slack,
    /// Discord account
    Discord,
    /// Custom account
    Custom,
}

/// Account status
#[derive(Debug, Clone, PartialEq)]
pub enum AccountStatus {
    /// Connected
    Connected,
    /// Connecting
    Connecting,
    /// Disconnected
    Disconnected,
    /// Error
    Error,
    /// Authentication failed
    AuthFailed,
}

/// Account
#[derive(Debug, Clone)]
pub struct Account {
    /// Account ID
    pub id: String,
    /// Account username
    pub username: String,
    /// Account password (encrypted)
    pub password: String,
    /// Account type
    pub account_type: AccountType,
    /// Account status
    pub status: AccountStatus,
    /// Account display name
    pub display_name: String,
    /// Account avatar path
    pub avatar_path: Option<String>,
    /// Account server/host
    pub server: String,
    /// Account port
    pub port: u16,
    /// Account use SSL/TLS
    pub use_ssl: bool,
    /// Account auto-connect
    pub auto_connect: bool,
    /// Account created timestamp
    pub created_at: u64,
    /// Account last connected timestamp
    pub last_connected: u64,
}

impl Account {
    /// Create new account
    pub fn new(id: String, username: String, account_type: AccountType) -> Self {
        Self {
            id,
            username,
            password: String::new(),
            account_type,
            status: AccountStatus::Disconnected,
            display_name: String::new(),
            avatar_path: None,
            server: String::new(),
            port: 0,
            use_ssl: false,
            auto_connect: false,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_connected: 0,
        }
    }
    
    /// Set password
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
    
    /// Set display name
    pub fn set_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
    }
    
    /// Set avatar path
    pub fn set_avatar(&mut self, avatar_path: String) {
        self.avatar_path = Some(avatar_path);
    }
    
    /// Set server
    pub fn set_server(&mut self, server: String, port: u16) {
        self.server = server;
        self.port = port;
    }
    
    /// Enable SSL/TLS
    pub fn enable_ssl(&mut self) {
        self.use_ssl = true;
    }
    
    /// Disable SSL/TLS
    pub fn disable_ssl(&mut self) {
        self.use_ssl = false;
    }
    
    /// Enable auto-connect
    pub fn enable_auto_connect(&mut self) {
        self.auto_connect = true;
    }
    
    /// Disable auto-connect
    pub fn disable_auto_connect(&mut self) {
        self.auto_connect = false;
    }
    
    /// Connect account
    pub fn connect(&mut self) -> bool {
        if self.status == AccountStatus::Disconnected {
            self.status = AccountStatus::Connected;
            self.last_connected = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            return true;
        }
        false
    }
    
    /// Disconnect account
    pub fn disconnect(&mut self) -> bool {
        if self.status == AccountStatus::Connected {
            self.status = AccountStatus::Disconnected;
            return true;
        }
        false
    }
    
    /// Set account status
    pub fn set_status(&mut self, status: AccountStatus) {
        self.status = status;
    }
    
    /// Check if account is connected
    pub fn is_connected(&self) -> bool {
        self.status == AccountStatus::Connected
    }
    
    /// Check if account is connecting
    pub fn is_connecting(&self) -> bool {
        self.status == AccountStatus::Connecting
    }
    
    /// Check if account is disconnected
    pub fn is_disconnected(&self) -> bool {
        self.status == AccountStatus::Disconnected
    }
    
    /// Check if account has error
    pub fn has_error(&self) -> bool {
        self.status == AccountStatus::Error || self.status == AccountStatus::AuthFailed
    }
    
    /// Get account age in days
    pub fn get_age_days(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        (now - self.created_at) / 86400
    }
    
    /// Get time since last connection in seconds
    pub fn get_time_since_last_connection(&self) -> u64 {
        if self.last_connected == 0 {
            return 0;
        }
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.last_connected
    }
    
    /// Get connection URL
    pub fn get_connection_url(&self) -> String {
        let protocol = match self.account_type {
            AccountType::XMPP => "xmpp",
            AccountType::IRC => "irc",
            AccountType::Matrix => "matrix",
            AccountType::Telegram => "telegram",
            AccountType::WhatsApp => "whatsapp",
            AccountType::Signal => "signal",
            AccountType::Slack => "slack",
            AccountType::Discord => "discord",
            AccountType::Custom => "custom",
        };
        
        if self.server.is_empty() {
            format!("{}://{}", protocol, self.username)
        } else {
            format!("{}://{}:{}", protocol, self.server, self.port)
        }
    }
}
