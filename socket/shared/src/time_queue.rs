use std::{cmp::Ordering, collections::BinaryHeap};

use super::Instant;

/// A queue for items marked by time, will only ever pop items from the queue if
/// the time passes
#[derive(Clone)]
pub struct TimeQueue<T: Eq + PartialEq> {
    queue: BinaryHeap<ItemContainer<T>>,
}

#[allow(clippy::new_without_default)]
impl<T: Eq + PartialEq> TimeQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::default(),
        }
    }
}

impl<T: Eq + PartialEq> TimeQueue<T> {
    /// Adds an item to the queue marked by time
    pub fn add_item(&mut self, instant: Instant, item: T) {
        self.queue.push(ItemContainer { instant, item });
    }

    /// Returns whether or not there is an item whose time has elapsed on the queue
    pub fn has_item(&self, now: &Instant) -> bool {
        if self.queue.is_empty() {
            return false;
        }
        if let Some(item) = self.queue.peek() {
            // item's instant has passed, so it's ready to be returned

            let will_pop = now.is_after(&item.instant);

            return will_pop;
        }
        false
    }

    /// Pops an item from the queue if it's time has elapsed
    pub fn pop_item(&mut self, now: &Instant) -> Option<T> {
        if self.has_item(now) {
            if let Some(container) = self.queue.pop() {
                return Some(container.item);
            }
        }
        None
    }

    /// Peeks at the top level item container on the queue
    pub fn peek_entry(&self) -> Option<&ItemContainer<T>> {
        self.queue.peek()
    }

    /// Returns the length of the underlying queue
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Checks if the underlying queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ItemContainer<T: Eq + PartialEq> {
    pub instant: Instant,
    pub item: T,
}

impl<T: Eq + PartialEq> Ord for ItemContainer<T> {
    fn cmp(&self, other: &ItemContainer<T>) -> Ordering {
        other.instant.cmp(&self.instant)
    }
}

impl<T: Eq + PartialEq> PartialOrd for ItemContainer<T> {
    fn partial_cmp(&self, other: &ItemContainer<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
