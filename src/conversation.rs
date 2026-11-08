//! Conversation definitions for aim.

use crate::message::Message;

/// Conversation type
#[derive(Debug, Clone, PartialEq)]
pub enum ConversationType {
    /// Direct conversation
    Direct,
    /// Group conversation
    Group,
    /// Channel conversation
    Channel,
    /// Broadcast conversation
    Broadcast,
}

/// Conversation participant
#[derive(Debug, Clone)]
pub struct Participant {
    /// Participant ID
    pub id: String,
    /// Participant name
    pub name: String,
    /// Participant role
    pub role: ParticipantRole,
    /// Participant joined timestamp
    pub joined_at: u64,
    /// Participant last active timestamp
    pub last_active: u64,
    /// Participant muted
    pub muted: bool,
}

/// Participant role
#[derive(Debug, Clone, PartialEq)]
pub enum ParticipantRole {
    /// Owner
    Owner,
    /// Admin
    Admin,
    /// Moderator
    Moderator,
    /// Member
    Member,
}

/// Conversation
#[derive(Debug, Clone)]
pub struct Conversation {
    /// Conversation ID
    pub id: String,
    /// Conversation name
    pub name: String,
    /// Conversation type
    pub conversation_type: ConversationType,
    /// Conversation participants
    pub participants: Vec<Participant>,
    /// Conversation messages
    pub messages: Vec<Message>,
    /// Conversation created timestamp
    pub created_at: u64,
    /// Conversation last activity timestamp
    pub last_activity: u64,
    /// Conversation unread count
    pub unread_count: usize,
    /// Conversation pinned
    pub pinned: bool,
    /// Conversation archived
    pub archived: bool,
    /// Conversation muted
    pub muted: bool,
}

impl Conversation {
    /// Create new conversation
    pub fn new(id: String, name: String, conversation_type: ConversationType) -> Self {
        Self {
            id,
            name,
            conversation_type,
            participants: vec![],
            messages: vec![],
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_activity: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            unread_count: 0,
            pinned: false,
            archived: false,
            muted: false,
        }
    }
    
    /// Add participant
    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant);
        self.update_last_activity();
    }
    
    /// Remove participant
    pub fn remove_participant(&mut self, participant_id: &str) -> bool {
        if let Some(pos) = self.participants.iter().position(|p| p.id == participant_id) {
            self.participants.remove(pos);
            self.update_last_activity();
            return true;
        }
        false
    }
    
    /// Get participant
    pub fn get_participant(&self, participant_id: &str) -> Option<&Participant> {
        self.participants.iter().find(|p| p.id == participant_id)
    }
    
    /// Get participant mutably
    pub fn get_participant_mut(&mut self, participant_id: &str) -> Option<&mut Participant> {
        self.participants.iter_mut().find(|p| p.id == participant_id)
    }
    
    /// Add message
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.unread_count += 1;
        self.update_last_activity();
    }
    
    /// Get message
    pub fn get_message(&self, message_id: &str) -> Option<&Message> {
        self.messages.iter().find(|m| m.id == message_id)
    }
    
    /// Get message mutably
    pub fn get_message_mut(&mut self, message_id: &str) -> Option<&mut Message> {
        self.messages.iter_mut().find(|m| m.id == message_id)
    }
    
    /// Remove message
    pub fn remove_message(&mut self, message_id: &str) -> bool {
        if let Some(pos) = self.messages.iter().position(|m| m.id == message_id) {
            self.messages.remove(pos);
            return true;
        }
        false
    }
    
    /// Mark as read
    pub fn mark_as_read(&mut self) {
        self.unread_count = 0;
        self.update_last_activity();
    }
    
    /// Increment unread count
    pub fn increment_unread(&mut self) {
        self.unread_count += 1;
    }
    
    /// Decrement unread count
    pub fn decrement_unread(&mut self) {
        if self.unread_count > 0 {
            self.unread_count -= 1;
        }
    }
    
    /// Pin conversation
    pub fn pin(&mut self) {
        self.pinned = true;
    }
    
    /// Unpin conversation
    pub fn unpin(&mut self) {
        self.pinned = false;
    }
    
    /// Archive conversation
    pub fn archive(&mut self) {
        self.archived = true;
    }
    
    /// Unarchive conversation
    pub fn unarchive(&mut self) {
        self.archived = false;
    }
    
    /// Mute conversation
    pub fn mute(&mut self) {
        self.muted = true;
    }
    
    /// Unmute conversation
    pub fn unmute(&mut self) {
        self.muted = false;
    }
    
    /// Update last activity timestamp
    fn update_last_activity(&mut self) {
        self.last_activity = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Get conversation age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Check if conversation is active
    pub fn is_active(&self) -> bool {
        !self.archived && !self.muted
    }
    
    /// Check if conversation has unread messages
    pub fn has_unread_messages(&self) -> bool {
        self.unread_count > 0
    }
    
    /// Get participant count
    pub fn get_participant_count(&self) -> usize {
        self.participants.len()
    }
    
    /// Get message count
    pub fn get_message_count(&self) -> usize {
        self.messages.len()
    }
    
    /// Get last message
    pub fn get_last_message(&self) -> Option<&Message> {
        self.messages.last()
    }
    
    /// Search messages
    pub fn search_messages(&self, query: &str) -> Vec<&Message> {
        let query_lower = query.to_lowercase();
        self.messages.iter()
            .filter(|m| m.content.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    /// Get messages by sender
    pub fn get_messages_by_sender(&self, sender_id: &str) -> Vec<&Message> {
        self.messages.iter()
            .filter(|m| m.sender_id == sender_id)
            .collect()
    }
    
    /// Get messages by type
    pub fn get_messages_by_type(&self, message_type: &crate::message::MessageType) -> Vec<&Message> {
        self.messages.iter()
            .filter(|m| m.message_type == *message_type)
            .collect()
    }
}
