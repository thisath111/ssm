use crate::utils::win32;

/// Large Pages (2MB / 4MB HugePages) & Compressed Memory Optimizer.
/// Minimizes Translation Lookaside Buffer (TLB) misses on CPU caches.
pub struct LargePageOptimizer;

impl LargePageOptimizer {
    /// Grants and verifies the SeLockMemoryPrivilege required for Large Page allocations.
    pub fn enable_large_pages() -> bool {
        win32::enable_privilege("SeLockMemoryPrivilege")
    }

    /// Queries minimum large page size supported by the current processor (usually 2MB).
    pub fn get_large_page_minimum() -> usize {
        unsafe {
            windows::Win32::System::Memory::GetLargePageMinimum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large_page_query() {
        let min_size = LargePageOptimizer::get_large_page_minimum();
        // Returns 0 if not supported or 2MB/4MB (e.g. 2097152)
        assert!(min_size == 0 || min_size >= 2 * 1024 * 1024);
    }
}
