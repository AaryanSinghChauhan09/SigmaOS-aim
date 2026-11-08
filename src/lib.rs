//! SigmaOS-aim — comprehensive instant messaging component.
//!
//! Provides instant messaging functionality with multi-protocol support,
//! contact management, conversation handling, and message delivery.

pub mod message;
pub mod conversation;
pub mod contact;
pub mod account;
pub mod protocol;
pub mod manager;
pub mod options;

pub use message::{Message, MessageType, MessageStatus, Attachment};
pub use conversation::{Conversation, ConversationType, Participant};
pub use contact::{Contact, ContactStatus, ContactGroup};
pub use account::{Account, AccountType, AccountStatus};
pub use protocol::{Protocol, ProtocolType, ProtocolStatus};
pub use manager::{MessageManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_message_type() {
        let types = vec![
            MessageType::Text,
            MessageType::Image,
            MessageType::Audio,
            MessageType::Video,
            MessageType::File,
            MessageType::Location,
            MessageType::Contact,
            MessageType::Sticker,
            MessageType::System,
        ];
        assert_eq!(types.len(), 9);
    }
    
    #[test]
    fn test_message_status() {
        let statuses = vec![
            MessageStatus::Sending,
            MessageStatus::Sent,
            MessageStatus::Delivered,
            MessageStatus::Read,
            MessageStatus::Failed,
        ];
        assert_eq!(statuses.len(), 5);
    }
    
    #[test]
    fn test_conversation_type() {
        let types = vec![
            ConversationType::Direct,
            ConversationType::Group,
            ConversationType::Channel,
            ConversationType::Broadcast,
        ];
        assert_eq!(types.len(), 4);
    }
    
    #[test]
    fn test_contact_status() {
        let statuses = vec![
            ContactStatus::Online,
            ContactStatus::Away,
            ContactStatus::Busy,
            ContactStatus::Offline,
            ContactStatus::Invisible,
        ];
        assert_eq!(statuses.len(), 5);
    }
    
    #[test]
    fn test_account_type() {
        let types = vec![
            AccountType::XMPP,
            AccountType::IRC,
            AccountType::Matrix,
            AccountType::Telegram,
            AccountType::WhatsApp,
            AccountType::Signal,
            AccountType::Slack,
            AccountType::Discord,
            AccountType::Custom,
        ];
        assert_eq!(types.len(), 9);
    }
    
    #[test]
    fn test_protocol_type() {
        let types = vec![
            ProtocolType::XMPP,
            ProtocolType::IRC,
            ProtocolType::Matrix,
            ProtocolType::Telegram,
            ProtocolType::WhatsApp,
            ProtocolType::Signal,
            ProtocolType::Slack,
            ProtocolType::Discord,
            ProtocolType::Custom,
        ];
        assert_eq!(types.len(), 9);
    }
    
    #[test]
    fn test_message_creation() {
        let message = Message::new(
            "msg-1".into(),
            "conv-1".into(),
            "user-1".into(),
            "Hello, world!".into(),
            MessageType::Text,
        );
        
        assert_eq!(message.id, "msg-1");
        assert_eq!(message.conversation_id, "conv-1");
        assert_eq!(message.sender_id, "user-1");
        assert_eq!(message.content, "Hello, world!");
        assert_eq!(message.message_type, MessageType::Text);
    }
    
    #[test]
    fn test_conversation_creation() {
        let conversation = Conversation::new(
            "conv-1".into(),
            "Direct Chat".into(),
            ConversationType::Direct,
        );
        
        assert_eq!(conversation.id, "conv-1");
        assert_eq!(conversation.name, "Direct Chat");
        assert_eq!(conversation.conversation_type, ConversationType::Direct);
    }
    
    #[test]
    fn test_contact_creation() {
        let contact = Contact::new(
            "contact-1".into(),
            "John Doe".into(),
            "john@example.com".into(),
        );
        
        assert_eq!(contact.id, "contact-1");
        assert_eq!(contact.name, "John Doe");
        assert_eq!(contact.status, ContactStatus::Online);
    }
    
    #[test]
    fn test_account_creation() {
        let account = Account::new(
            "account-1".into(),
            "user@example.com".into(),
            AccountType::XMPP,
        );
        
        assert_eq!(account.id, "account-1");
        assert_eq!(account.username, "user@example.com");
        assert_eq!(account.account_type, AccountType::XMPP);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = MessageManager::new();
        assert!(manager.list_conversations().is_empty());
        assert!(manager.list_contacts().is_empty());
        assert!(manager.list_accounts().is_empty());
    }
    
    #[test]
    fn test_create_conversation() {
        let mut manager = MessageManager::new();
        let conv_id = manager.create_conversation(
            "Direct Chat".into(),
            ConversationType::Direct,
        );
        
        assert!(manager.get_conversation(&conv_id).is_some());
        assert_eq!(manager.list_conversations().len(), 1);
    }
    
    #[test]
    fn test_send_message() {
        let mut manager = MessageManager::new();
        let conv_id = manager.create_conversation(
            "Direct Chat".into(),
            ConversationType::Direct,
        );
        
        let msg_id = manager.send_message(
            conv_id.clone(),
            "user-1".into(),
            "Hello!".into(),
            MessageType::Text,
        );
        
        assert!(manager.get_message(&msg_id).is_some());
        
        let conv = manager.get_conversation(&conv_id).unwrap();
        assert_eq!(conv.messages.len(), 1);
    }
    
    #[test]
    fn test_add_contact() {
        let mut manager = MessageManager::new();
        let contact_id = manager.add_contact(
            "John Doe".into(),
            "john@example.com".into(),
        );
        
        assert!(manager.get_contact(&contact_id).is_some());
        assert_eq!(manager.list_contacts().len(), 1);
    }
    
    #[test]
    fn test_add_account() {
        let mut manager = MessageManager::new();
        let account_id = manager.add_account(
            "user@example.com".into(),
            AccountType::XMPP,
        );
        
        assert!(manager.get_account(&account_id).is_some());
        assert_eq!(manager.list_accounts().len(), 1);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = MessageManager::new();
        
        let conv_id = manager.create_conversation(
            "Direct Chat".into(),
            ConversationType::Direct,
        );
        manager.send_message(conv_id, "user-1".into(), "Hello!".into(), MessageType::Text);
        
        let export = manager.export_data();
        
        let mut manager2 = MessageManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_conversations().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = MessageManager::new();
        
        manager.create_conversation("Chat1".into(), ConversationType::Direct);
        manager.create_conversation("Chat2".into(), ConversationType::Group);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_conversations, 2);
        assert_eq!(stats.total_contacts, 0);
        assert_eq!(stats.total_accounts, 0);
    }
}
