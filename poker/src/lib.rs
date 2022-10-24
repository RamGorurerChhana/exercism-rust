use std::cmp::Ordering;
use std::str::FromStr;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // create Hand for each input strings
    let mut hands = hands.iter().map(|&h| Hand::new(h)).collect::<Vec<_>>();
    // sort the hands by descending order of ranks
    hands.sort_unstable_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
    let mut v = vec![];
    // take from the beginning and continue until hands are equal rank
    for i in 0..hands.len() {
        if i == 0 || hands[i - 1].rank == hands[i].rank {
            v.push(hands[i].hand_str);
        } else {
            break;
        }
    }

    v
}

type CardValue = u8;
type FiveCards = [Card; 5];

#[derive(Debug, PartialEq)]
enum Rank {
    StraightFlush(CardValue),
    FourOfAKind(CardValue, CardValue),
    FullHouse(CardValue, CardValue),
    Flush(Box<Rank>),
    Straight(CardValue),
    ThreeOfAKind(CardValue, CardValue, CardValue),
    TwoPair(CardValue, CardValue, CardValue),
    OnePair(CardValue, CardValue, CardValue, CardValue),
    HighCard(CardValue, CardValue, CardValue, CardValue, CardValue),
}

/// implement PartialOrd for Rank so that hands can be sorted
/// based on their ranks
impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            // both hands have Straigt Flush
            (Self::StraightFlush(v1), Self::StraightFlush(v2)) => Some(v1.cmp(v2)),
            // one hand has straight Flush
            (Self::StraightFlush(_), _) => Some(Ordering::Greater),
            (_, Self::StraightFlush(_)) => Some(Ordering::Less),
            // Both hands have FourOfAKind
            (Self::FourOfAKind(m1, m2), Self::FourOfAKind(n1, n2)) => {
                ordering_multiple_cards(&[(*m1, *n1), (*m2, *n2)])
            }
            // one hand has FourOfAKind
            (Self::FourOfAKind(_, _), _) => Some(Ordering::Greater),
            (_, Self::FourOfAKind(_, _)) => Some(Ordering::Less),
            // both hands have FullHouse
            (Self::FullHouse(m1, m2), Self::FullHouse(n1, n2)) => {
                ordering_multiple_cards(&[(*m1, *n1), (*m2, *n2)])
            }
            // only one hand has FullHouse
            (Self::FullHouse(_, _), _) => Some(Ordering::Greater),
            (_, Self::FullHouse(_, _)) => Some(Ordering::Less),
            // Both hands have Flash
            (Self::Flush(b1), Self::Flush(b2)) => b1.partial_cmp(b2),
            // Only one hand has Flush
            (Self::Flush(_), _) => Some(Ordering::Greater),
            (_, Self::Flush(_)) => Some(Ordering::Less),
            // Both hands have Straight
            (Self::Straight(v1), Self::Straight(v2)) => Some(v1.cmp(v2)),
            // Only one hand has Straight
            (Self::Straight(_), _) => Some(Ordering::Greater),
            (_, Self::Straight(_)) => Some(Ordering::Less),
            // Both hands have ThreeOfAKind
            (Self::ThreeOfAKind(m1, m2, m3), Self::ThreeOfAKind(n1, n2, n3)) => {
                ordering_multiple_cards(&[(*m1, *n1), (*m2, *n2), (*m3, *n3)])
            }
            // Only one hand has ThreeOfAKind
            (Self::ThreeOfAKind(_, _, _), _) => Some(Ordering::Greater),
            (_, Self::ThreeOfAKind(_, _, _)) => Some(Ordering::Less),
            // Both hands have TwoPair
            (Self::TwoPair(m1, m2, m3), Self::TwoPair(n1, n2, n3)) => {
                ordering_multiple_cards(&[(*m1, *n1), (*m2, *n2), (*m3, *n3)])
            }
            // only one hand has TwoPair
            (Self::TwoPair(_, _, _), _) => Some(Ordering::Greater),
            (_, Self::TwoPair(_, _, _)) => Some(Ordering::Less),
            // Both hands have OnePair
            (Self::OnePair(m1, m2, m3, m4), Self::OnePair(n1, n2, n3, n4)) => {
                ordering_multiple_cards(&[(*m1, *n1), (*m2, *n2), (*m3, *n3), (*m4, *n4)])
            }
            // Only one hands have OnePair
            (Self::OnePair(_, _, _, _), _) => Some(Ordering::Greater),
            (_, Self::OnePair(_, _, _, _)) => Some(Ordering::Less),
            // Both hands have HighCard
            (Self::HighCard(m1, m2, m3, m4, m5), Self::HighCard(n1, n2, n3, n4, n5)) => {
                ordering_multiple_cards(&[
                    (*m1, *n1),
                    (*m2, *n2),
                    (*m3, *n3),
                    (*m4, *n4),
                    (*m5, *n5),
                ])
            }
        }
    }
}
// ordering multiple pairs of cards
// comparing one pair at a time
fn ordering_multiple_cards(cards: &[(CardValue, CardValue)]) -> Option<Ordering> {
    for card in cards {
        match card.0.cmp(&card.1) {
            Ordering::Greater => return Some(Ordering::Greater),
            Ordering::Less => return Some(Ordering::Less),
            Ordering::Equal => {}
        };
    }
    Some(Ordering::Equal)
}

impl Rank {
    fn assign_rank(cards: &FiveCards) -> Self {
        let has_flush = is_flush(cards);
        let mut all_values = cards.iter().map(|c| c.value).collect::<Vec<_>>();
        all_values.sort_unstable();
        // Check for Straight Flush and Straight
        if let Some(v) = check_straight(&all_values) {
            if has_flush {
                return Self::StraightFlush(v);
            } else {
                return Self::Straight(v);
            }
        }
        // check FourOfAKind
        if let Some((v1, v2)) = check_four_of_a_kind(&all_values) {
            return Self::FourOfAKind(v1, v2);
        }
        // check FullHouse
        if let Some((v1, v2)) = check_full_house(&all_values) {
            return Self::FullHouse(v1, v2);
        }

        // check ThreeOfAKind
        let rank = if let Some((v1, v2, v3)) = check_three_of_a_kind(&all_values) {
            Self::ThreeOfAKind(v1, v2, v3)
            // check TwoPair
        } else if let Some((v1, v2, v3)) = check_two_pairs(&all_values) {
            Self::TwoPair(v1, v2, v3)
            // check OnePair
        } else if let Some((v1, v2, v3, v4)) = check_one_pair(&all_values) {
            Self::OnePair(v1, v2, v3, v4)
        } else {
            // default HighCard
            Self::HighCard(
                all_values[4],
                all_values[3],
                all_values[2],
                all_values[1],
                all_values[0],
            )
        };
        if has_flush {
            Self::Flush(Box::new(rank))
        } else {
            rank
        }
    }
}

// if all 5 cards have same suite
fn is_flush(cards: &FiveCards) -> bool {
    cards.iter().all(|c| c.suite == cards[0].suite)
}

// check if has straight
// cards are sorted by CardValue ascending
fn check_straight(values: &[CardValue]) -> Option<CardValue> {
    // attempt to make a straight
    if values[0] + 1 == values[1]
        && values[1] + 1 == values[2]
        && values[2] + 1 == values[3]
        && values[3] + 1 == values[4]
    {
        return Some(values[4]);
    }
    // attempt to make an aces with putting ace = 1
    if [2, 3, 4, 5, 14].into_iter().all(|n| values.contains(&n)) {
        return Some(5);
    }
    // straight not possible
    None
}

// check FourOfAKind
// cards are sorted by CardValue ascending
fn check_four_of_a_kind(values: &[CardValue]) -> Option<(CardValue, CardValue)> {
    if values[0] == values[3] {
        return Some((values[0], values[4]));
    }
    if values[1] == values[4] {
        return Some((values[1], values[0]));
    }

    None
}

// check FullHouse
// cards are sorted by CardValue ascending
fn check_full_house(values: &[CardValue]) -> Option<(CardValue, CardValue)> {
    if values[0] == values[2] && values[3] == values[4] {
        return Some((values[0], values[3]));
    }
    if values[0] == values[1] && values[2] == values[4] {
        return Some((values[2], values[0]));
    }

    None
}

// check ThreeOfAKind
// cards are sorted by CardValue ascending
fn check_three_of_a_kind(values: &[CardValue]) -> Option<(CardValue, CardValue, CardValue)> {
    if values[0] == values[2] {
        Some((values[0], values[4], values[3]))
    } else if values[1] == values[3] {
        Some((values[1], values[4], values[0]))
    } else if values[2] == values[4] {
        Some((values[2], values[1], values[0]))
    } else {
        None
    }
}

// check TwoPairs
// cards are sorted by CardValue ascending
fn check_two_pairs(values: &[CardValue]) -> Option<(CardValue, CardValue, CardValue)> {
    if values[0] == values[1] && values[2] == values[3] {
        Some((values[2], values[0], values[4]))
    } else if values[1] == values[2] && values[3] == values[4] {
        Some((values[3], values[1], values[0]))
    } else if values[0] == values[1] && values[3] == values[4] {
        Some((values[3], values[1], values[2]))
    } else {
        None
    }
}

// check OnePair
// cards are sorted by CardValue ascending
fn check_one_pair(values: &[CardValue]) -> Option<(CardValue, CardValue, CardValue, CardValue)> {
    if values[0] == values[1] {
        Some((values[0], values[4], values[3], values[2]))
    } else if values[1] == values[2] {
        Some((values[1], values[4], values[3], values[0]))
    } else if values[2] == values[3] {
        Some((values[2], values[4], values[1], values[0]))
    } else if values[3] == values[4] {
        Some((values[3], values[2], values[1], values[0]))
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suite {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suite: Suite,
    value: CardValue,
}

impl Card {
    fn new(suite: Suite, value: CardValue) -> Self {
        Self { suite, value }
    }
}

/// Create a card instance from the given string
/// it will assign the card value and the suite
impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // except last char take from the beginning
        let value_str = &s[0..s.len() - 1];
        // last char represents the suite
        let suite = &s[s.len() - 1..s.len()];
        // cards from 2 - 10 will parse successfully
        // other 4 cards assign value manually
        let value = match value_str.parse::<CardValue>() {
            Ok(v) => v,
            Err(_) => match value_str {
                "J" => 11,
                "Q" => 12,
                "K" => 13,
                "A" => 14,
                _ => unreachable!(),
            },
        };
        // match suite and create Card instance
        match suite {
            "S" => Ok(Self::new(Suite::Spades, value)),
            "H" => Ok(Self::new(Suite::Hearts, value)),
            "D" => Ok(Self::new(Suite::Diamonds, value)),
            "C" => Ok(Self::new(Suite::Clubs, value)),
            _ => Err("unreachable"),
        }
    }
}

#[derive(Debug)]
struct Hand<'a> {
    hand_str: &'a str,
    rank: Rank,
}

impl<'a> Hand<'a> {
    fn new(hand_str: &'a str) -> Self {
        // each hand contains 5 cards
        // split by ' ' and then convert to FiveCards
        let hands = hand_str
            .split(' ')
            .map(|s| Card::from_str(s).unwrap())
            .collect::<Vec<_>>();
        if hands.len() != 5 {
            panic!("Invalid hand");
        }
        // assign the highest possible rank for the Hand
        let rank = Rank::assign_rank(&[hands[0], hands[1], hands[2], hands[3], hands[4]]);
        Self { hand_str, rank }
    }
}
