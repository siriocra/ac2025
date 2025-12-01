pub fn part1(input:String) -> i32 {
    return 0;
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(String::from("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82")), 3);
    }
}