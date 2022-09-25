// future references :)
use std::alloc::{alloc, Layout};

pub fn split_into_sections<T>(slice: &[T], sections: usize) -> Vec<&[T]> {
    let slice_len = slice.len();
    if slice.len() <= sections {
        return vec![slice];
    }

    let remaining = slice_len % sections;
    let segments_amount = (slice_len / sections) + if remaining == 0 { 0 } else { 1 };

    // sorry rust gangsters, i have to do it for performance reasons
    let ptr = unsafe {
        let layout = Layout::array::<&[T]>(segments_amount).expect("allocation failed");
        alloc(layout) as *mut &[T]
    };

    let mut i = 1;
    while i <= segments_amount {
        let index = i - 1;
        let max_index = if i == segments_amount {
            slice_len - 1
        } else {
            (i * sections) - 1
        };
        unsafe {
            let segment_slice: &[T] = slice.get_unchecked((index * sections)..=max_index);
            *ptr.add(index) = segment_slice;
        }
        i += 1;
    }

    unsafe { Vec::from_raw_parts(ptr, segments_amount, segments_amount) }
}
