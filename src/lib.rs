// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-preseed: preseed component model.
//!
//! Real implementation: preseed_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreseedItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreseedError {
    EmptyPreseedItemName,
    DuplicatePreseedItem(String),
    EmptyEventSubject,
}

impl fmt::Display for PreseedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPreseedItemName => write!(f, "preseed_item name must not be empty"),
            Self::DuplicatePreseedItem(id) => write!(f, "duplicate preseed_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for PreseedError {}

pub struct Preseed {
    pub name: String,
    preseed_items: BTreeMap<String, PreseedItem>,
    events: BTreeMap<String, Event>,
}

impl Preseed {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            preseed_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_preseed_item(&mut self, item: PreseedItem) -> Result<(), PreseedError> {
        if item.name.trim().is_empty() {
            return Err(PreseedError::EmptyPreseedItemName);
        }
        let id = item.id.clone();
        if self.preseed_items.contains_key(&id) {
            return Err(PreseedError::DuplicatePreseedItem(id.clone()));
        }
        self.preseed_items.insert(id, item);
        Ok(())
    }

    pub fn preseed_item_count(&self) -> usize {
        self.preseed_items.len()
    }

    pub fn get_preseed_item(&self, id: &str) -> Option<&PreseedItem> {
        self.preseed_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), PreseedError> {
        if event.subject.trim().is_empty() {
            return Err(PreseedError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_preseed_item(id: &str, name: &str) -> PreseedItem {
        PreseedItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Preseed::new("Preseed");
        assert_eq!(m.name, "Preseed");
        assert_eq!(m.preseed_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_preseed_item() {
        let mut m = Preseed::new("Preseed");
        m.add_preseed_item(sample_preseed_item("a1", "Alpha")).unwrap();
        assert_eq!(m.preseed_item_count(), 1);
        assert!(m.get_preseed_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Preseed::new("Preseed");
        assert!(m.add_preseed_item(sample_preseed_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Preseed::new("Preseed");
        m.add_preseed_item(sample_preseed_item("x", "One")).unwrap();
        assert!(m.add_preseed_item(sample_preseed_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Preseed::new("Preseed");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
