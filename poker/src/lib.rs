/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hand = hands.to_vec();

    hand.sort();

    hand
}


// Poker hands with different categories ranking (from high to low)
// Straight Flash 
// Four of a kind 
// Full House
// Flush
// Straight
// Three of a kind
// Two Pair
// One Pair
// High Card

// If both hands have Straight Flash then highest card value from both hands are compoared. If both hands have same high card then its tie. 
// If both hands have Four of a kind then card value of the common cards are compared. If common card value are same then value of the remaining card matters. 
// If both hands have Full House then value of 3 of a kind card compared first, then value of 2 of a kind card compared. 
// If both hands have Flush then value of the high card compared followed by next high card. 
// If both hands have Straight then value of the high card compared followed by next high card.
// If both hands have Three of a kind then value of the common card compared first, followed by value of the high card
// If both hands have two pairs then value of common cards are compared first followed by next high card 
// If both hands have one pair then value of common cards are compared first followed by next high card
// If both hands have only high cards then value of the high cards are compared followed by next high card 


// Enum Rank
// StraightFlush(u8) -- value denoting the high card value
// FourOfAKind(u8, u8) -- first value of the common card, second value of the remaining card
// FullHouse(u8, u8) - first value of 3 common card, second value of pair card
// Flush(Box<Rank>) - value points to a rank (E.g- Flush(Box<TwoPair(9, 4, 2)>))
// Straight(u8) - value denoting the high card value 
// TheeOfAKind(u8, u8, u8) - common card value, next high card, remaining card
// TwoPair(u8, u8, u8) - high pair card value, next pair card value, remaining card
// OnePair(u8, u8, u8, u8) - pair card value, next high card, next high card, next high card
// HighCard(u8, u8, u8, u8, u8) - card values from high to low


// Enum to denote Card
// Card
// Spades(CardValue)
// Hearts(CardValue)
// Diamonds(CardValue)
// Clubs(CardValue)

// Enum to denote card value
// CardValue
// One = 1
// Two = 2
// Three = 3
// Four = 4
// Five = 5
// Six = 6
// Seven = 7
// Eight = 8
// Nine = 9
// Ten = 10
// Jack = 11
// Queen = 12
// King = 13
// Ace = 14

// struct to denote Hand 
// Hand
// cards: [Card;5],
// rank: Rank


// TIPS: Since Ace can be treated as Ace and one both
// we can consider as two cards and take the best 5 cards out of 6 cards

// panic incase of errorneous input
// not putting any error handling now

// Parsing Strategy:
// all slice of &str will have exactly 5 &str when splitted by <space>, otherwise panic
// all &str will be either 2 chars or 3 chars 
// last char is suite
// remaining chars from the begininng will denote value 
// Implement FromStr trait for CardValue 
// Implement FromStr trait for Card which will internally get CardValue 

// Implement FromStr for Hand struct
// which will first construct array of card [Card;5]
// then calculate the rank of the hand
// instantiate the Hand

// Implement PartialEq for Hand struct 
// so that it can be compared while sorting 

// sort the original Vec<&str> ascending passing a function to sort_by function 
// which will convert both &str to Hand and then compare 
//
// create a new Vec<&str> to return 
// pop a item from the vec if there is no item already in the new vec then insert
// pop again compare the value with the item in the vec 
// if low then break and return 
// otherwise insert and continue again 

// That's pretty much how the logic would go. 
// HARE KRISHNA. 🙏 
// GOD SAVE ME. 😭 

















