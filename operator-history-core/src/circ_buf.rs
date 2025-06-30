use bytemuck::{Pod, Zeroable};
use jito_bytemuck::types::{PodBool, PodU64};
use operator_history_sdk::error::OperatorHistoryError;
use shank::ShankType;

use crate::{operator_history_entry::OperatorHistotyEntry, OPERATOR_HISTORY_ENTRY_MAX_ITEMS};

pub fn find_insert_position(arr: &[OperatorHistotyEntry], idx: usize, epoch: u16) -> Option<usize> {
    let len = arr.len();
    if len == 0 {
        return None;
    }

    let insert_pos =
        if idx != len - 1 && arr[idx + 1].epoch() == OperatorHistotyEntry::default().epoch() {
            // If the circ buf still has default values in it, we do a normal binary search without factoring for wraparound.
            let len = idx + 1;
            let mut left = 0;
            let mut right = len;
            while left < right {
                let mid = (left + right) / 2;
                match arr[mid].epoch().cmp(&epoch) {
                    std::cmp::Ordering::Equal => return None,
                    std::cmp::Ordering::Less => left = mid + 1,
                    std::cmp::Ordering::Greater => right = mid,
                }
            }
            left % arr.len()
        } else {
            // Binary search with wraparound
            let mut left = 0;
            let mut right = len;
            while left < right {
                let mid = (left + right) / 2;
                // idx + 1 is the index of the smallest epoch in the array
                let mid_idx = ((idx + 1) + mid) % len;
                match arr[mid_idx].epoch().cmp(&epoch) {
                    std::cmp::Ordering::Equal => return None,
                    std::cmp::Ordering::Less => left = mid + 1,
                    std::cmp::Ordering::Greater => right = mid,
                }
            }
            ((idx + 1) + left) % len
        };

    if arr[insert_pos].epoch() == epoch {
        return None;
    }

    Some(insert_pos)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, ShankType)]
#[repr(C)]
pub struct CircBuf {
    /// Index
    index: PodU64,

    /// Is empty
    is_empty: PodBool,

    /// Array of operator history
    arr: [OperatorHistotyEntry; OPERATOR_HISTORY_ENTRY_MAX_ITEMS],

    /// Reserved space
    reserved_space: [u8; 328],
}

impl CircBuf {
    pub fn new() -> Self {
        Self {
            index: PodU64::from(0),
            is_empty: PodBool::from_bool(false),
            arr: [OperatorHistotyEntry::default(); OPERATOR_HISTORY_ENTRY_MAX_ITEMS],
            reserved_space: [0; 328],
        }
    }

    /// Index
    pub fn index(&self) -> u64 {
        self.index.into()
    }

    /// Is empty
    pub fn is_empty(&self) -> bool {
        self.is_empty.into()
    }

    /// Push an [`OperatorHistoryEntry`] item
    pub fn push(&mut self, item: OperatorHistotyEntry) {
        let index = (self.index() + 1) % self.arr.len() as u64;

        self.index = PodU64::from(index);
        self.arr[self.index() as usize] = item;
        self.is_empty = PodBool::from_bool(false);
    }

    /// Fetch last [`OperatorHistoryEntry`] element
    pub fn last(&self) -> Option<&OperatorHistotyEntry> {
        if self.is_empty() {
            None
        } else {
            Some(&self.arr[self.index() as usize])
        }
    }

    /// Fetch last [`OperatorHistoryEntry`] element
    pub fn last_mut(&mut self) -> Option<&mut OperatorHistotyEntry> {
        if self.is_empty() {
            None
        } else {
            Some(&mut self.arr[self.index() as usize])
        }
    }

    /// Fetch mutable array of [`OperatorHistoryEntry`]
    pub fn arr_mut(&mut self) -> &mut [OperatorHistotyEntry] {
        &mut self.arr
    }

    /// Given a new entry and epoch, inserts the entry into the buffer in sorted order
    /// Will not insert if the epoch is out of range or already exists in the buffer
    pub fn insert(
        &mut self,
        entry: OperatorHistotyEntry,
        epoch: u16,
    ) -> Result<(), OperatorHistoryError> {
        if self.is_empty() {
            return Err(OperatorHistoryError::EpochOutOfRange);
        }

        // Find the lowest epoch in the buffer to ensure the new epoch is valid
        let min_epoch = {
            let next_i = (self.index() as usize + 1) % self.arr.len();
            if self.arr[next_i].epoch() == OperatorHistotyEntry::default().epoch() {
                self.arr[0].epoch()
            } else {
                self.arr[next_i].epoch()
            }
        };

        // If epoch is less than min_epoch or greater than max_epoch in the buffer, return error
        if epoch < min_epoch || epoch > self.arr[self.index() as usize].epoch() {
            return Err(OperatorHistoryError::EpochOutOfRange);
        }

        let insert_pos = find_insert_position(&self.arr, self.index() as usize, epoch)
            .ok_or(OperatorHistoryError::DuplicateEpoch)?;

        // If idx < insert_pos, the shifting needs to wrap around
        let end_index = if self.index() < insert_pos as u64 {
            self.index() as usize + self.arr.len()
        } else {
            self.index() as usize
        };

        // Shift all elements to the right to make space for the new entry, starting with current idx
        for i in (insert_pos..=end_index).rev() {
            let i = i % self.arr.len();
            let next_i = (i + 1) % self.arr.len();
            self.arr[next_i] = self.arr[i];
        }

        self.arr[insert_pos] = entry;

        self.index = PodU64::from((self.index() + 1) % self.arr.len() as u64);

        Ok(())
    }
}
