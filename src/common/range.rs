use std::{
    cmp::{max, min},
    ops::Range,
};

/// range_intersect takes range `range` and range `x`. This calculates the three possible
/// overlaps of `range` with respects to `x`
///
/// e.g.
/// ```
/// # use advent_of_code_2023::common::range_intersect;
/// // x does not intersect, two possible ways, before or after
/// assert_eq!(range_intersect(10..15, &(0..7)), [None, None, Some(10..15)]);
/// assert_eq!(range_intersect(10..15, &(17..30)), [Some(10..15), None, None]);
///
/// // x intersects completely
/// assert_eq!(range_intersect(10..15, &(10..15)), [None, Some(10..15), None]);
///
/// // x is within both the start and end of range
/// assert_eq!(range_intersect(10..15, &(12..13)), [Some(10..12), Some(12..13), Some(13..15)]);
///
/// // x intersects to the beginning or end of range
/// assert_eq!(range_intersect(10..15, &(0..13)), [None, Some(10..13), Some(13..15)]);
/// assert_eq!(range_intersect(10..15, &(13..20)), [Some(10..13), Some(13..15), None]);
/// ```
///
pub fn range_intersect<T: Ord + Copy>(range: Range<T>, x: &Range<T>) -> [Option<Range<T>>; 3] {
    let before = range.start..min(range.end, x.start);
    let inter = max(range.start, x.start)..min(x.end, range.end);
    let after = max(x.end, range.start)..range.end;

    [
        (before.end > before.start).then_some(before),
        (inter.end > inter.start).then_some(inter),
        (after.end > after.start).then_some(after),
    ]
}
