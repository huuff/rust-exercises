use std::collections::VecDeque;

pub struct History<K, V> {
    pub entries: VecDeque<HistoryEntry<K, V>>,
}

impl <K, V> History<K, V> {
    pub fn new() -> Self {
	Self {
	    entries: VecDeque::new(),
	}
    }

    pub fn push(&mut self, key: K, value: V) {
	self.entries.push_front(HistoryEntry { key, value });
    }
}

pub struct HistoryEntry<Key, Value> {
    pub key: Key,
    pub value: Value,
}
