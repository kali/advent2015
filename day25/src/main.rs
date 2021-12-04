fn main() {
    dbg!(code(2981, 3075));
}

fn position(row: usize, col: usize) -> usize {
    let row = row - 1;
    let col = col - 1;
    let diag = row + col;
    diag * (diag + 1) / 2 + col + 1
}

fn code(row: usize, col: usize) -> usize {
    let mut v = 20151125;
    for _ in 1..position(row, col) {
        v = (v * 252533) % 33554393;
    }
    v
}

#[test]
fn test_position() {
    assert_eq!(position(1, 1), 1);
    assert_eq!(position(2, 1), 2);
    assert_eq!(position(1, 2), 3);
    assert_eq!(position(4, 3), 18);
}

#[test]
fn test() {
    let expected = [
        [20151125, 18749137, 17289845, 30943339, 10071777, 33511524],
        [31916031, 21629792, 16929656, 7726640, 15514188, 4041754],
        [16080970, 8057251, 1601130, 7981243, 11661866, 16474243],
        [24592653, 32451966, 21345942, 9380097, 10600672, 31527494],
        [77061, 17552253, 28094349, 6899651, 9250759, 31663883],
        [33071741, 6796745, 25397450, 24659492, 1534922, 27995004],
    ];

    for row in 1..=6 {
        for col in 1..=6 {
            let exp = expected[row - 1][col - 1];
            assert_eq!(exp, code(row, col));
        }
    }
}
