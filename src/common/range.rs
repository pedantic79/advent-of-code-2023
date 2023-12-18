use std::{
    cmp::{max, min},
    ops::Range,
};

pub fn range_overlap<T: Ord + Copy>(a: Range<T>, b: Range<T>) -> [Option<Range<T>>; 3] {
    let before = a.start..min(a.end, b.start);
    let inter = max(a.start, b.start)..min(b.end, a.end);
    let after = max(b.end, a.start)..a.end;

    [
        (before.end > before.start).then_some(before),
        (inter.end > inter.start).then_some(inter),
        (after.end > after.start).then_some(after),
    ]
}
