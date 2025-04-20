/// Reorder `slice` such that `cond(element)` is:
/// *   `true` for everything in the first returned slice
/// *   `false` for everything in the last returned slice
///
/// As this partitioning is unstable, elements in either subslice may not retain their original order.
#[allow(dead_code)]
pub fn in_place_unstable_split<T>(slice: &mut [T], cond: impl FnMut(&T) -> bool) -> (&mut [T], &mut [T]) {
    let i = in_place_unstable(slice, cond);
    slice.split_at_mut(i)
}

/// Reorder `slice` such that `cond(element)` is:
/// *   `true` for everything before the returned index
/// *   `false` for everything at or after the returned index
///
/// As this partitioning is unstable, elements on either side of the index may not retain their original order.
pub fn in_place_unstable<T>(slice: &mut [T], mut cond: impl FnMut(&T) -> bool) -> usize {
    let mut part = 0;
    while part < slice.len() && cond(&slice[part]) { part += 1 }
    if part == slice.len() { return part }

    let mut last = slice.len();
    last -= 1;
    slice.swap(part, last);

    while part < last {
        if cond(&slice[part]) {
            part += 1;
        } else {
            last -= 1;
            slice.swap(part, last);
        }
    }

    part
}

#[test] fn part() {
    use crate::prelude::*;

    let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (even, odd) = partition::in_place_unstable_split(&mut a[..], |i| i%2==0);
    even.sort();
    odd.sort();
    assert_eq!([2, 4, 6, 8, 10], even);
    assert_eq!([1, 3, 5, 7, 9], odd);
}
