use std::{str::FromStr, collections::HashMap, fmt::Debug};


// type hand consisting of a fixed tuple of char of size 5
// type Hand = (char, char, char, char, char);

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Card(char);

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // cards strength order A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2

        // Order for question 1
        let mut card_order = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

        card_order.reverse();
        let self_index = card_order.iter().position(|&x| x == self.0).unwrap();
        let other_index = card_order.iter().position(|&x| x == other.0).unwrap();

        self_index.partial_cmp(&other_index)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}


#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    cards: Vec<Card>,
    pub bid: u32,
    occurences: HashMap<Card, u32>,
}


impl FromStr for Hand {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (s, bid_str) = line.split_once(" ").unwrap();
        if s.len() != 5 {
            return Err(());
        }

        let mut chars = line.chars();
        let first = chars.next().unwrap();
        let second = chars.next().unwrap();
        let third = chars.next().unwrap();
        let fourth = chars.next().unwrap();
        let fifth = chars.next().unwrap();

        let mut hand = Hand {
            cards: vec![Card(first), Card(second), Card(third), Card(fourth), Card(fifth)],
            bid: bid_str.parse::<u32>().unwrap(),
            occurences: HashMap::new(),
        };
        hand.occurences = hand.get_occurences();
        Ok(hand)
    }
}

impl Hand {
    fn get_occurences(&self) -> HashMap<Card, u32> {
        let mut occurences: HashMap<Card, u32> = HashMap::new();
        for card in &self.cards {
            match occurences.get_mut(card) {
                Some(count) => *count += 1,
                None => {
                    occurences.insert(*card, 1);
                }
            }
            
        }
        occurences
    }

    fn five_of_a_kind(&self) -> bool {
        for (_, count) in &self.occurences {
            if *count == 5 {
                return true;
            }
        }
        false
    }

    fn four_of_a_kind(&self) -> bool {
        for (_, count) in &self.occurences {
            if *count == 4 {
                return true;
            }
        }
        false
        
    }

    fn full_house(&self) -> bool {
        let mut three_of_a_kind = false;
        let mut two_of_a_kind = false;
        for (_, count) in &self.occurences {
            if *count == 3 {
                three_of_a_kind = true;
            }
            if *count == 2 {
                two_of_a_kind = true;
            }
        }
        three_of_a_kind && two_of_a_kind
    }

    fn three_of_a_kind(&self) -> bool {
        for (_, count) in &self.occurences {
            if *count == 3 {
                return true;
            }
        }
        false
    }

    fn two_pairs(&self) -> bool {
        let mut pairs = 0;
        for (_, count) in &self.occurences {
            if *count == 2 {
                pairs += 1;
            }
        }
        pairs == 2
    }

    fn one_pair(&self) -> bool {
        let mut pairs = 0;
        for (_, count) in &self.occurences {
            if *count == 2 {
                pairs += 1;
            }
        }
        pairs == 1
    }
}


impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_conditions = vec![
            self.five_of_a_kind(),
            self.four_of_a_kind(),
            self.full_house(),
            self.three_of_a_kind(),
            self.two_pairs(),
            self.one_pair(),
            true,
        ];
        let other_conditions = vec![
            other.five_of_a_kind(),
            other.four_of_a_kind(),
            other.full_house(),
            other.three_of_a_kind(),
            other.two_pairs(),
            other.one_pair(),
            true,
        ];

        let self_index = self_conditions.iter().position(|&x| x == true).unwrap();
        let other_index = other_conditions.iter().position(|&x| x == true).unwrap();
        if self_index != other_index {
            return other_index.partial_cmp(&self_index);
        } 

        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            if self_card != other_card {
                return self_card.partial_cmp(&other_card);
            }
        } 
        None
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}
