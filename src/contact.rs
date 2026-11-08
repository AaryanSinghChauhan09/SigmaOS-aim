//! Contact definitions for aim.

/// Contact status
#[derive(Debug, Clone, PartialEq)]
pub enum ContactStatus {
    /// Online
    Online,
    /// Away
    Away,
    /// Busy
    Busy,
    /// Offline
    Offline,
    /// Invisible
    Invisible,
}

/// Contact group
#[derive(Debug, Clone)]
pub struct ContactGroup {
    /// Group ID
    pub id: String,
    /// Group name
    pub name: String,
    /// Group description
    pub description: String,
    /// Group created timestamp
    pub created_at: u64,
    /// Group contacts
    pub contacts: Vec<String>,
}

/// Contact
#[derive(Debug, Clone)]
pub struct Contact {
    /// Contact ID
    pub id: String,
    /// Contact name
    pub name: String,
    /// Contact email
    pub email: String,
    /// Contact phone
    pub phone: Option<String>,
    /// Contact avatar path
    pub avatar_path: Option<String>,
    /// Contact status
    pub status: ContactStatus,
    /// Contact groups
    pub groups: Vec<String>,
    /// Contact notes
    pub notes: String,
    /// Contact created timestamp
    pub created_at: u64,
    /// Contact last seen timestamp
    pub last_seen: u64,
    /// Contact blocked
    pub blocked: bool,
    /// Contact favorite
    pub favorite: bool,
}

impl Contact {
    /// Create new contact
    pub fn new(id: String, name: String, email: String) -> Self {
        Self {
            id,
            name,
            email,
            phone: None,
            avatar_path: None,
            status: ContactStatus::Online,
            groups: vec![],
            notes: String::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_seen: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            blocked: false,
            favorite: false,
        }
    }
    
    /// Set phone number
    pub fn set_phone(&mut self, phone: String) {
        self.phone = Some(phone);
    }
    
    /// Set avatar path
    pub fn set_avatar(&mut self, avatar_path: String) {
        self.avatar_path = Some(avatar_path);
    }
    
    /// Set status
    pub fn set_status(&mut self, status: ContactStatus) {
        self.status = status;
    }
    
    /// Add to group
    pub fn add_to_group(&mut self, group_id: String) {
        if !self.groups.contains(&group_id) {
            self.groups.push(group_id);
        }
    }
    
    /// Remove from group
    pub fn remove_from_group(&mut self, group_id: &str) -> bool {
        if let Some(pos) = self.groups.iter().position(|g| g == group_id) {
            self.groups.remove(pos);
            return true;
        }
        false
    }
    
    /// Set notes
    pub fn set_notes(&mut self, notes: String) {
        self.notes = notes;
    }
    
    /// Update last seen
    pub fn update_last_seen(&mut self) {
        self.last_seen = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Block contact
    pub fn block(&mut self) {
        self.blocked = true;
    }
    
    /// Unblock contact
    pub fn unblock(&mut self) {
        self.blocked = false;
    }
    
    /// Mark as favorite
    pub fn mark_favorite(&mut self) {
        self.favorite = true;
    }
    
    /// Unmark as favorite
    pub fn unmark_favorite(&mut self) {
        self.favorite = false;
    }
    
    /// Check if contact is online
    pub fn is_online(&self) -> bool {
        self.status == ContactStatus::Online
    }
    
    /// Check if contact is available
    pub fn is_available(&self) -> bool {
        self.status == ContactStatus::Online || self.status == ContactStatus::Away
    }
    
    /// Get contact age in days
    pub fn get_age_days(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        (now - self.created_at) / 86400
    }
    
    /// Check if contact is new
    pub fn is_new(&self) -> bool {
        self.get_age_days() < 7
    }
    
    /// Get last seen time
    pub fn get_last_seen(&self) -> u64 {
        self.last_seen
    }
    
    /// Get contact initials
    pub fn get_initials(&self) -> String {
        self.name.split_whitespace()
            .map(|word| word.chars().next().unwrap_or(' '))
            .collect::<String>()
            .to_uppercase()
    }
}
