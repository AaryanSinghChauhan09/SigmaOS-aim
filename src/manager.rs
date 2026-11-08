//! Message manager for aim.

use std::collections::HashMap;
use crate::message::{Message, MessageType, MessageStatus};
use crate::conversation::{Conversation, ConversationType, Participant};
use crate::contact::{Contact, ContactStatus};
use crate::account::{Account, AccountType, AccountStatus};
use crate::protocol::{Protocol, ProtocolType, ProtocolStatus};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Auto-connect accounts
    pub auto_connect: bool,
    /// Enable message encryption
    pub enable_encryption: bool,
    /// Enable message delivery receipts
    pub enable_delivery_receipts: bool,
    /// Enable message read receipts
    pub enable_read_receipts: bool,
    /// Max message history
    pub max_message_history: usize,
    /// Max attachment size in MB
    pub max_attachment_size_mb: u64,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            auto_connect: true,
            enable_encryption: true,
            enable_delivery_receipts: true,
            enable_read_receipts: true,
            max_message_history: 10000,
            max_attachment_size_mb: 100,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total conversations
    pub total_conversations: usize,
    /// Total contacts
    pub total_contacts: usize,
    /// Total accounts
    pub total_accounts: usize,
    /// Total protocols
    pub total_protocols: usize,
    /// Total messages
    pub total_messages: usize,
    /// Online contacts
    pub online_contacts: usize,
    /// Connected accounts
    pub connected_accounts: usize,
    /// Active protocols
    pub active_protocols: usize,
}

/// Message manager
#[derive(Debug, Clone)]
pub struct MessageManager {
    /// Conversations
    conversations: HashMap<String, Conversation>,
    /// Contacts
    contacts: HashMap<String, Contact>,
    /// Accounts
    accounts: HashMap<String, Account>,
    /// Protocols
    protocols: HashMap<String, Protocol>,
    /// Messages
    messages: HashMap<String, Message>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
}

impl Default for MessageManager {
    fn default() -> Self {
        Self {
            conversations: HashMap::new(),
            contacts: HashMap::new(),
            accounts: HashMap::new(),
            protocols: HashMap::new(),
            messages: HashMap::new(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_conversations: 0,
                total_contacts: 0,
                total_accounts: 0,
                total_protocols: 0,
                total_messages: 0,
                online_contacts: 0,
                connected_accounts: 0,
                active_protocols: 0,
            },
        }
    }
}

impl MessageManager {
    /// Create new message manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        let mut manager = Self::default();
        manager.options = options;
        manager
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        // Add default protocols
        self.add_protocol(Protocol::new("XMPP".into(), ProtocolType::XMPP));
        self.add_protocol(Protocol::new("IRC".into(), ProtocolType::IRC));
        self.add_protocol(Protocol::new("Matrix".into(), ProtocolType::Matrix));
        
        self.update_statistics();
        
        if self.options.auto_connect {
            self.connect_all_accounts();
        }
    }
    
    /// Create conversation
    pub fn create_conversation(&mut self, name: String, conversation_type: ConversationType) -> String {
        let id = format!("conv-{}", self.conversations.len() + 1);
        let conversation = Conversation::new(id.clone(), name, conversation_type);
        
        self.conversations.insert(id.clone(), conversation);
        self.update_statistics();
        id
    }
    
    /// Get conversation
    pub fn get_conversation(&self, conversation_id: &str) -> Option<&Conversation> {
        self.conversations.get(conversation_id)
    }
    
    /// Get conversation mutably
    pub fn get_conversation_mut(&mut self, conversation_id: &str) -> Option<&mut Conversation> {
        self.conversations.get_mut(conversation_id)
    }
    
    /// Get all conversations
    pub fn list_conversations(&self) -> Vec<&Conversation> {
        self.conversations.values().collect()
    }
    
    /// Delete conversation
    pub fn delete_conversation(&mut self, conversation_id: &str) -> bool {
        if self.conversations.remove(conversation_id).is_some() {
            // Remove all messages in conversation
            self.messages.retain(|_, m| m.conversation_id != conversation_id);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add participant to conversation
    pub fn add_participant_to_conversation(&mut self, conversation_id: &str, participant: Participant) -> bool {
        if let Some(conv) = self.conversations.get_mut(conversation_id) {
            conv.add_participant(participant);
            return true;
        }
        false
    }
    
    /// Remove participant from conversation
    pub fn remove_participant_from_conversation(&mut self, conversation_id: &str, participant_id: &str) -> bool {
        if let Some(conv) = self.conversations.get_mut(conversation_id) {
            if conv.remove_participant(participant_id) {
                return true;
            }
        }
        false
    }
    
    /// Send message
    pub fn send_message(&mut self, conversation_id: String, sender_id: String, content: String, message_type: MessageType) -> String {
        let message_id = format!("msg-{}", self.messages.len() + 1);
        let message = Message::new(
            message_id.clone(),
            conversation_id.clone(),
            sender_id,
            content,
            message_type,
        );
        
        // Store message
        self.messages.insert(message_id.clone(), message.clone());
        
        // Add to conversation
        if let Some(conv) = self.conversations.get_mut(&conversation_id) {
            conv.add_message(message);
        }
        
        // Mark as sent
        if let Some(msg) = self.messages.get_mut(&message_id) {
            msg.mark_sent();
        }
        
        self.update_statistics();
        message_id
    }
    
    /// Get message
    pub fn get_message(&self, message_id: &str) -> Option<&Message> {
        self.messages.get(message_id)
    }
    
    /// Get message mutably
    pub fn get_message_mut(&mut self, message_id: &str) -> Option<&mut Message> {
        self.messages.get_mut(message_id)
    }
    
    /// Delete message
    pub fn delete_message(&mut self, message_id: &str) -> bool {
        if self.messages.remove(message_id).is_some() {
            // Remove from conversation
            for conv in self.conversations.values_mut() {
                conv.remove_message(message_id);
            }
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add contact
    pub fn add_contact(&mut self, name: String, email: String) -> String {
        let id = format!("contact-{}", self.contacts.len() + 1);
        let contact = Contact::new(id.clone(), name, email);
        
        self.contacts.insert(id.clone(), contact);
        self.update_statistics();
        id
    }
    
    /// Get contact
    pub fn get_contact(&self, contact_id: &str) -> Option<&Contact> {
        self.contacts.get(contact_id)
    }
    
    /// Get contact mutably
    pub fn get_contact_mut(&mut self, contact_id: &str) -> Option<&mut Contact> {
        self.contacts.get_mut(contact_id)
    }
    
    /// Get all contacts
    pub fn list_contacts(&self) -> Vec<&Contact> {
        self.contacts.values().collect()
    }
    
    /// Delete contact
    pub fn delete_contact(&mut self, contact_id: &str) -> bool {
        if self.contacts.remove(contact_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Update contact status
    pub fn update_contact_status(&mut self, contact_id: &str, status: ContactStatus) -> bool {
        if let Some(contact) = self.contacts.get_mut(contact_id) {
            contact.set_status(status);
            contact.update_last_seen();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add account
    pub fn add_account(&mut self, username: String, account_type: AccountType) -> String {
        let id = format!("account-{}", self.accounts.len() + 1);
        let account = Account::new(id.clone(), username, account_type);
        
        self.accounts.insert(id.clone(), account);
        self.update_statistics();
        id
    }
    
    /// Get account
    pub fn get_account(&self, account_id: &str) -> Option<&Account> {
        self.accounts.get(account_id)
    }
    
    /// Get account mutably
    pub fn get_account_mut(&mut self, account_id: &str) -> Option<&mut Account> {
        self.accounts.get_mut(account_id)
    }
    
    /// Get all accounts
    pub fn list_accounts(&self) -> Vec<&Account> {
        self.accounts.values().collect()
    }
    
    /// Delete account
    pub fn delete_account(&mut self, account_id: &str) -> bool {
        if self.accounts.remove(account_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Connect account
    pub fn connect_account(&mut self, account_id: &str) -> bool {
        if let Some(account) = self.accounts.get_mut(account_id) {
            if account.connect() {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Disconnect account
    pub fn disconnect_account(&mut self, account_id: &str) -> bool {
        if let Some(account) = self.accounts.get_mut(account_id) {
            if account.disconnect() {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Connect all accounts
    pub fn connect_all_accounts(&mut self) -> usize {
        let mut connected = 0;
        for account in self.accounts.values_mut() {
            if account.auto_connect && account.connect() {
                connected += 1;
            }
        }
        self.update_statistics();
        connected
    }
    
    /// Disconnect all accounts
    pub fn disconnect_all_accounts(&mut self) -> usize {
        let mut disconnected = 0;
        for account in self.accounts.values_mut() {
            if account.disconnect() {
                disconnected += 1;
            }
        }
        self.update_statistics();
        disconnected
    }
    
    /// Add protocol
    pub fn add_protocol(&mut self, protocol: Protocol) -> String {
        let id = format!("protocol-{}", self.protocols.len() + 1);
        let mut protocol = protocol;
        protocol.name = format!("{}-{}", protocol.name, id);
        
        self.protocols.insert(id.clone(), protocol);
        self.update_statistics();
        id
    }
    
    /// Get protocol
    pub fn get_protocol(&self, protocol_id: &str) -> Option<&Protocol> {
        self.protocols.get(protocol_id)
    }
    
    /// Get protocol mutably
    pub fn get_protocol_mut(&mut self, protocol_id: &str) -> Option<&mut Protocol> {
        self.protocols.get_mut(protocol_id)
    }
    
    /// Get all protocols
    pub fn list_protocols(&self) -> Vec<&Protocol> {
        self.protocols.values().collect()
    }
    
    /// Delete protocol
    pub fn delete_protocol(&mut self, protocol_id: &str) -> bool {
        if self.protocols.remove(protocol_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Activate protocol
    pub fn activate_protocol(&mut self, protocol_id: &str) -> bool {
        if let Some(protocol) = self.protocols.get_mut(protocol_id) {
            protocol.activate();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Deactivate protocol
    pub fn deactivate_protocol(&mut self, protocol_id: &str) -> bool {
        if let Some(protocol) = self.protocols.get_mut(protocol_id) {
            protocol.deactivate();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Search conversations
    pub fn search_conversations(&self, query: &str) -> Vec<&Conversation> {
        let query_lower = query.to_lowercase();
        self.conversations.values()
            .filter(|c| c.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    /// Search contacts
    pub fn search_contacts(&self, query: &str) -> Vec<&Contact> {
        let query_lower = query.to_lowercase();
        self.contacts.values()
            .filter(|c| c.name.to_lowercase().contains(&query_lower) || 
                       c.email.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    /// Search messages
    pub fn search_messages(&self, query: &str) -> Vec<&Message> {
        let query_lower = query.to_lowercase();
        self.messages.values()
            .filter(|m| m.content.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    /// Get conversations by type
    pub fn get_conversations_by_type(&self, conversation_type: &ConversationType) -> Vec<&Conversation> {
        self.conversations.values()
            .filter(|c| c.conversation_type == *conversation_type)
            .collect()
    }
    
    /// Get contacts by status
    pub fn get_contacts_by_status(&self, status: &ContactStatus) -> Vec<&Contact> {
        self.contacts.values()
            .filter(|c| c.status == *status)
            .collect()
    }
    
    /// Get accounts by type
    pub fn get_accounts_by_type(&self, account_type: &AccountType) -> Vec<&Account> {
        self.accounts.values()
            .filter(|a| a.account_type == *account_type)
            .collect()
    }
    
    /// Get protocols by type
    pub fn get_protocols_by_type(&self, protocol_type: &ProtocolType) -> Vec<&Protocol> {
        self.protocols.values()
            .filter(|p| p.protocol_type == *protocol_type)
            .collect()
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_conversations = self.conversations.len();
        self.statistics.total_contacts = self.contacts.len();
        self.statistics.total_accounts = self.accounts.len();
        self.statistics.total_protocols = self.protocols.len();
        self.statistics.total_messages = self.messages.len();
        
        self.statistics.online_contacts = self.contacts.values()
            .filter(|c| c.status == ContactStatus::Online)
            .count();
        
        self.statistics.connected_accounts = self.accounts.values()
            .filter(|a| a.status == AccountStatus::Connected)
            .count();
        
        self.statistics.active_protocols = self.protocols.values()
            .filter(|p| p.status == ProtocolStatus::Active)
            .count();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            conversations: self.conversations.values().cloned().collect(),
            contacts: self.contacts.values().cloned().collect(),
            accounts: self.accounts.values().cloned().collect(),
            protocols: self.protocols.values().cloned().collect(),
            messages: self.messages.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.conversations.clear();
        for conv in export.conversations {
            self.conversations.insert(conv.id.clone(), conv);
        }
        
        self.contacts.clear();
        for contact in export.contacts {
            self.contacts.insert(contact.id.clone(), contact);
        }
        
        self.accounts.clear();
        for account in export.accounts {
            self.accounts.insert(account.id.clone(), account);
        }
        
        self.protocols.clear();
        for protocol in export.protocols {
            self.protocols.insert(protocol.name.clone(), protocol);
        }
        
        self.messages.clear();
        for message in export.messages {
            self.messages.insert(message.id.clone(), message);
        }
        
        self.update_statistics();
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Conversations
    pub conversations: Vec<Conversation>,
    /// Contacts
    pub contacts: Vec<Contact>,
    /// Accounts
    pub accounts: Vec<Account>,
    /// Protocols
    pub protocols: Vec<Protocol>,
    /// Messages
    pub messages: Vec<Message>,
}
