use std::collections::VecDeque;

pub struct History<K, V> {
    // TODO: Is there a specialized stack type in rust?
    pub entries: VecDeque<HistoryEntry<K, V>>,
}

impl <K, V> History<K, V> {
    pub fn new() -> Self {
	Self {
	    entries: VecDeque::new(),
	}
    }

    pub fn push(&mut self, entry: HistoryEntry<K, V>) {
	self.entries.push_front(entry);
    }
}

pub struct HistoryEntry<Key, Value> {
    pub key: Key,
    pub value: Value,
}
