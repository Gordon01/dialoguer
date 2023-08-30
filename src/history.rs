use crate::prompts::input::HistoryContainer;

/// Trait for history handling.
pub trait History {
    /// This is called with the current position that should
    /// be read from history. The `pos` represents the number
    /// of times the `Up`/`Down` arrow key has been pressed.
    /// This would normally be used as an index to some sort
    /// of vector.  If the `pos` does not have an entry, [`None`](Option::None)
    /// should be returned.
    fn read(&self, pos: usize) -> Option<String>;

    /// This is called with the next value you should store
    /// in history at the first location. Normally history
    /// is implemented as a FIFO queue.
    fn write(&mut self, val: String);
}

impl History for HistoryContainer<'_> {
    fn read(&self, pos: usize) -> Option<String> {
        match self {
            HistoryContainer::Referenced(h) => h.read(pos),
            HistoryContainer::Infinite(vd) => vd.get(pos).cloned(),
        }
    }

    fn write(&mut self, val: String) {
        match self {
            HistoryContainer::Referenced(h) => h.write(val),
            HistoryContainer::Infinite(vd) => vd.push_front(val),
        }
    }
}
