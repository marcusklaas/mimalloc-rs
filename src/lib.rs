use memmap::MmapMut;

struct Segment {
    pages: [Page; 64],
    page_areas: [PageArea; 64],
}

struct Page {
    free: *mut Block,
    used: usize,
    capacity: usize,
    next: *mut Page,
}

#[repr(C)]
struct Block {
    next: *mut Block,
}

impl PageArea {
    /// The `len` argument specificies how many words the
    /// elements are.
    fn init(&mut self, len: usize) {
        /// We do a simple linear initialization. This should be
        /// fine for now. Random initialization may be better for
        /// security reasons.
        unsafe {
            let mut ptr: *mut u64 = std::mem::transmute(&mut self.inner);
            let mut next: *mut _ = ptr.offset(len as isize);
            while next < ptr.offset(8 * 1024) {
                *ptr = next as u64;
                ptr = next;
                next = ptr.offset(len as isize);
            }
        }
    }
}

struct PageArea {
    inner: [u8; 64 * 1024],
}

struct MiMalloc;

fn malloc_small(len: usize) -> *mut u8 {
    std::ptr::null::<u8>() as *mut u8  
}

fn malloc_generic(len: usize) -> *mut u8 {
    std::ptr::null::<u8>() as *mut u8  
}

fn malloc_in_page(page: &mut Page, len: usize) -> *mut u8 {
    let block = page.free;
    if block.is_null() {
        malloc_generic(len)
    } else {
        unsafe {
            page.free = (*block).next;
            page.used += 1;
        }
        block as *mut u8
    }
}

#[cfg(test)]
mod tests {
    use memmap::MmapMut;
    use std::ops::Deref;

    #[test]
    fn it_works() {
        let map = MmapMut::map_anon(10);
        panic!("{:?}", map.unwrap().deref());
    }
}
