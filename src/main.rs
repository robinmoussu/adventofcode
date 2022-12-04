use assert2::assert;

enum Kind {
    Rock,
    Paper,
    Scissor,
}

fn score(yours: Kind, their: Kind) -> i32 {
    use Kind::*;
    (match yours {
        Rock => 1,
        Paper => 2,
        Scissor => 3,
    }) + match (yours, their) {
        (Rock, Scissor) | (Paper, Rock) | (Scissor, Paper) => 6,
        (Rock, Rock) | (Paper, Paper) | (Scissor, Scissor) => 3,
        (Scissor, Rock) | (Rock, Paper) | (Paper, Scissor) => 0,
    }
}

fn full_score(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            use Kind::*;

            let mut c = line.chars();
            let their = match c.next() {
                Some('A') => Rock,
                Some('B') => Paper,
                Some('C') => Scissor,
                x => panic!("invalid input: {:?}", x),
            };
            assert!(c.next() == Some(' '));
            let yours = match c.next() {
                Some('X') => Rock,
                Some('Y') => Paper,
                Some('Z') => Scissor,
                x => panic!("invalid input: {:?}", x),
            };
            assert!(c.next().is_none());
            score(yours, their)
        })
        .sum()
}

fn main() {
    println!(
        "day 2 part 1: {:?}",
        full_score(include_str!("../input-02.txt"))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;
    use indoc::indoc;

    #[test]
    fn test_score() {
        use Kind::*;
        assert!(score(Rock, Rock) == 1 + 3);
        assert!(score(Rock, Paper) == 1 + 0);
        assert!(score(Rock, Scissor) == 1 + 6);
        assert!(score(Paper, Rock) == 2 + 6);
        assert!(score(Paper, Paper) == 2 + 3);
        assert!(score(Paper, Scissor) == 2 + 0);
        assert!(score(Scissor, Rock) == 3 + 0);
        assert!(score(Scissor, Paper) == 3 + 6);
        assert!(score(Scissor, Scissor) == 3 + 3);
    }

    #[test]
    fn test_example() {
        assert!(
            full_score(indoc! {"
            A Y
            B X
            C Z
        "}) == 15
        );
    }
}
