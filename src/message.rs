//! Message definitions for aim.

/// Message type
#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    /// Text message
    Text,
    /// Image message
    Image,
    /// Audio message
    Audio,
    /// Video message
    Video,
    /// File message
    File,
    /// Location message
    Location,
    /// Contact message
    Contact,
    /// Sticker message
    Sticker,
    /// System message
    System,
}

/// Message status
#[derive(Debug, Clone, PartialEq)]
pub enum MessageStatus {
    /// Message is being sent
    Sending,
    /// Message sent
    Sent,
    /// Message delivered
    Delivered,
    /// Message read
    Read,
    /// Message failed
    Failed,
}

/// Message attachment
#[derive(Debug, Clone)]
pub struct Attachment {
    /// Attachment ID
    pub id: String,
    /// Attachment name
    pub name: String,
    /// Attachment path
    pub path: String,
    /// Attachment size in bytes
    pub size: u64,
    /// Attachment MIME type
    pub mime_type: String,
    /// Attachment thumbnail path
    pub thumbnail_path: Option<String>,
    /// Attachment upload timestamp
    pub uploaded_at: u64,
}

/// Message
#[derive(Debug, Clone)]
pub struct Message {
    /// Message ID
    pub id: String,
    /// Conversation ID
    pub conversation_id: String,
    /// Sender ID
    pub sender_id: String,
    /// Message content
    pub content: String,
    /// Message type
    pub message_type: MessageType,
    /// Message status
    pub status: MessageStatus,
    /// Message timestamp
    pub timestamp: u64,
    /// Message attachments
    pub attachments: Vec<Attachment>,
    /// Message reply to ID
    pub reply_to: Option<String>,
    /// Message edited
    pub edited: bool,
    /// Message deleted
    pub deleted: bool,
    /// Message read by
    pub read_by: Vec<String>,
    /// Message delivered to
    pub delivered_to: Vec<String>,
}

impl Message {
    /// Create new message
    pub fn new(
        id: String,
        conversation_id: String,
        sender_id: String,
        content: String,
        message_type: MessageType,
    ) -> Self {
        Self {
            id,
            conversation_id,
            sender_id,
            content,
            message_type,
            status: MessageStatus::Sending,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            attachments: vec![],
            reply_to: None,
            edited: false,
            deleted: false,
            read_by: vec![],
            delivered_to: vec![],
        }
    }
    
    /// Add attachment
    pub fn add_attachment(&mut self, attachment: Attachment) {
        self.attachments.push(attachment);
    }
    
    /// Remove attachment
    pub fn remove_attachment(&mut self, attachment_id: &str) -> bool {
        if let Some(pos) = self.attachments.iter().position(|a| a.id == attachment_id) {
            self.attachments.remove(pos);
            return true;
        }
        false
    }
    
    /// Get attachment
    pub fn get_attachment(&self, attachment_id: &str) -> Option<&Attachment> {
        self.attachments.iter().find(|a| a.id == attachment_id)
    }
    
    /// Set reply to
    pub fn set_reply_to(&mut self, reply_to_id: String) {
        self.reply_to = Some(reply_to_id);
    }
    
    /// Mark as edited
    pub fn mark_edited(&mut self) {
        self.edited = true;
    }
    
    /// Mark as deleted
    pub fn mark_deleted(&mut self) {
        self.deleted = true;
    }
    
    /// Mark as read by user
    pub fn mark_read_by(&mut self, user_id: String) {
        if !self.read_by.contains(&user_id) {
            self.read_by.push(user_id);
        }
        self.status = MessageStatus::Read;
    }
    
    /// Mark as delivered to user
    pub fn mark_delivered_to(&mut self, user_id: String) {
        if !self.delivered_to.contains(&user_id) {
            self.delivered_to.push(user_id);
        }
        if self.status == MessageStatus::Sent {
            self.status = MessageStatus::Delivered;
        }
    }
    
    /// Mark as sent
    pub fn mark_sent(&mut self) {
        self.status = MessageStatus::Sent;
    }
    
    /// Mark as failed
    pub fn mark_failed(&mut self) {
        self.status = MessageStatus::Failed;
    }
    
    /// Check if message is read
    pub fn is_read(&self) -> bool {
        self.status == MessageStatus::Read
    }
    
    /// Check if message is delivered
    pub fn is_delivered(&self) -> bool {
        self.status == MessageStatus::Delivered || self.status == MessageStatus::Read
    }
    
    /// Check if message is sent
    pub fn is_sent(&self) -> bool {
        self.status == MessageStatus::Sent || self.is_delivered()
    }
    
    /// Check if message has attachments
    pub fn has_attachments(&self) -> bool {
        !self.attachments.is_empty()
    }
    
    /// Get total attachment size
    pub fn get_total_attachment_size(&self) -> u64 {
        self.attachments.iter().map(|a| a.size).sum()
    }
    
    /// Get message age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.timestamp
    }
    
    /// Check if message is older than specified seconds
    pub fn is_older_than(&self, seconds: u64) -> bool {
        self.get_age_seconds() > seconds
    }
    
    /// Get message content preview
    pub fn get_content_preview(&self, max_length: usize) -> String {
        if self.content.len() > max_length {
            format!("{}...", &self.content[..max_length])
        } else {
            self.content.clone()
        }
    }
}
