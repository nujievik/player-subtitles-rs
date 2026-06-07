mod common;

use common::*;
use player_subtitles::*;

macro_rules! iter_slice_test {
    ($ty:ident, $fn:ident, $istr:expr, $lines:expr) => {
        #[test]
        fn $fn() {
            let mut blines = $ty::from_str($istr);
            for s in $lines {
                assert_eq!(blines.next().unwrap(), s.as_bytes());
            }
            assert!(blines.next().is_none());
        }
    };
}

iter_slice_test!(ByteLines, iter_slice, "0\n1\r2\r\n3", ["0", "1", "2", "3"]);
iter_slice_test!(
    ByteLines,
    iter_slice_with_empty_lines,
    "\n\r\r\n",
    ["", "", ""]
);

#[test]
fn iter_file() {
    let mut blines = ByteLines::open_file(data("four_lines.txt")).unwrap();
    for s in ["0", "1", "2", "3"] {
        assert_eq!(blines.next().unwrap(), s.as_bytes());
    }
    assert!(blines.next().is_none());
}
