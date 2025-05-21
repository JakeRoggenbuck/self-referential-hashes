fn main() {
    let start: i64 = 4_000_000;
    let end: i64 = 6_000_000_000;

    let binding = (start..end).collect::<Vec<_>>();
}
