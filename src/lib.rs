
pub fn run<'a>(args: &[String]) -> Result<Vec<u32>, &'static str> {
    let cards = &args[1];
    let cards: Vec<char> = cards.chars().collect();
    game(cards.clone())
}

fn game(cards: Vec<char>) -> Result<Vec<u32>, &'static str> {
    if let Err(_) = validate_zero_island(cards.clone()) {
        return Err("Unsolvable!");
    }
    let mut result: Vec<u32> = Vec::new();

    let mut i: usize = 0;
    let mut seen = vec![vec![]; 0];

    let mut cards_h = cards.clone();

    for card in cards {
        let mut v = false;
        if card == '1' {
            result.push(i as u32);
            cards_h[i] = '.';
            v = true;
        }
        if let Ok(solution) = game_helper(result.clone(), cards_h.clone(), &mut seen) {
            return Ok(solution);
        }
        if v {
            cards_h[i] = '1';
            result.pop();
        }
        i += 1;
    }
    Err("Invalid, no solution")
}

fn game_helper(
    result: Vec<u32>,
    cards: Vec<char>,
    seen: &mut Vec<Vec<char>>,
) -> Result<Vec<u32>, &'static str> {
    let mut result_h = result.clone();
    let mut cards_h = cards.clone();

    if has_seen_comb(cards.clone(), seen.clone()) {
        return Err("Seen this combination before!");
    } else {
        seen.push(cards.clone());
        let mut i: usize = 0;
        for card in cards {
            let mut v = false;
            if card == '1' {
                cards_h[i] = '.';
                if i as i32 - 1 >= 0 {
                    cards_h = flip_adjacent(cards_h.clone(), i - 1);
                }
                if i + 1 < cards_h.len() {
                    cards_h = flip_adjacent(cards_h.clone(), i + 1);
                }
                v = true;
                result_h.push(i as u32);
            }
            if validate_won(cards_h.clone()) {
                return Ok(result_h.clone());
            }
            if let Ok(solution) = game_helper(result_h.clone(), cards_h.clone(), seen) {
                return Ok(solution);
            }
            if v {
                cards_h[i] = '1';
                result_h.pop();
                if i as i32 - 1 >= 0 {
                    cards_h = flip_adjacent(cards_h.clone(), i - 1);
                }
                if i + 1 < cards_h.len() {
                    cards_h = flip_adjacent(cards_h.clone(), i + 1);
                }
            }
            i += 1;
        }
    }
    Err("couldn't find a solution")
}

fn flip_adjacent(cards: Vec<char>, index: usize) -> Vec<char> {
    let mut c = cards.clone();
    if let Some(value) = c.get(index) {
        if *value == '1' {
            c[index] = '0';
        } else if *value == '0' {
            c[index] = '1';
        }
    }
    c
}

fn validate_won(cards: Vec<char>) -> bool {
    for card in cards {
        if card != '.' {
            return false;
        }
    }
    true
}

fn has_seen_comb(cards: Vec<char>, seen: Vec<Vec<char>>) -> bool {
    for seen_comb in seen {
        if is_same_comb(cards.clone(), seen_comb.clone()) {
            return true;
        }
    }
    false
}

fn is_same_comb(cards: Vec<char>, seen_comb: Vec<char>) -> bool {
    let mut i = 0;
    for card in cards {
        if card != seen_comb[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn validate_zero_island(cards: Vec<char>) -> Result<(), &'static str> {
    let split_cards = split_at_dot(cards);
    for card_set in split_cards {
        if validate_card_set(&card_set) {
            return Err("Invalid card set");
        }
    }
    Ok(())
}

fn split_at_dot(cards: Vec<char>) -> Vec<Vec<char>> {
    let mut res = Vec::new();
    let mut curr = Vec::new();
    for card in cards {
        if card == '.' {
            res.push(curr);
            curr = Vec::new();
        } else {
            curr.push(card);
        }
    }
    res.push(curr);
    res
}

fn validate_card_set(card_set: &[char]) -> bool {
    if card_set.len() == 0 {
        return false;
    }
    for card in card_set {
        if *card == '1' {
            return false;
        }
    }
    true
}
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn validate_invalid() {
        let cards = vec!['1', '0', '.', '0', '0', '0'];
        let res = validate_zero_island(cards);
        assert_eq!(res, Err("Invalid"));
    }

    #[test]
    fn validate_valid() {
        let cards = vec!['1', '1', '0', '1'];
        let res = validate_zero_island(cards);
        assert_eq!(res, Ok(()));
    }

    #[test]
    fn validate_won_game() {
        let cards = vec!['.', '.', '.'];
        let res = validate_won(cards);
        assert_eq!(res, true);
    }

    #[test]
    fn validate_game_not_won() {
        let cards = vec!['.', '1', '.', '0'];
        let res = validate_won(cards);
        assert_eq!(res, false);
    }

    #[test]
    fn validate_has_seen() {
        let cards = vec!['.', '1', '0', '1'];
        let seen = vec![vec!['.', '1', '0', '1'], vec!['0', '1', '0', '0']];
        let res = has_seen_comb(cards, seen);
        assert_eq!(res, true);
    }

    #[test]
    fn validate_not_seen() {
        let cards = vec!['.', '1', '0', '1'];
        let seen = vec![vec!['.', '0', '0', '1'], vec!['0', '1', '0', '0']];
        let res = has_seen_comb(cards, seen);
        assert_eq!(res, false);
    }
}
