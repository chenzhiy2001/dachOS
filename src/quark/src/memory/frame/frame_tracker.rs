
use crate::memory::{address::*, FRAME_ALLOCATOR, PAGE_SIZE};

pub struct FrameTracker(pub(super) PhysicalPageNumber);

impl FrameTracker{
    pub fn address(&self) -> PhysicalAddress{
        self.0.into()
    }
    pub fn page_number(&self) -> PhysicalPageNumber{
        self.0
    }
}


/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::Deref for FrameTracker {
    type Target = [u8; PAGE_SIZE];
    fn deref(&self) -> &Self::Target {
        self.page_number().deref_kernel()
    }
}

/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::DerefMut for FrameTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.page_number().deref_kernel()
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self){
        FRAME_ALLOCATOR.lock().dealloc(self);
    }
}