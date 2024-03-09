use rand::seq::SliceRandom;
use std::fmt;

#[derive(Clone)]
pub struct Stack {
    cards: Vec<i8>,
}

impl Stack {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        cards.append(&mut vec![-2; 5]);
        cards.append(&mut vec![0; 15]);
        cards.append(&mut vec![-1; 10]);
        cards.append(&mut vec![1; 10]);
        cards.append(&mut vec![2; 10]);
        cards.append(&mut vec![3; 10]);
        cards.append(&mut vec![4; 10]);
        cards.append(&mut vec![5; 10]);
        cards.append(&mut vec![6; 10]);
        cards.append(&mut vec![7; 10]);
        cards.append(&mut vec![8; 10]);
        cards.append(&mut vec![9; 10]);
        cards.append(&mut vec![10; 10]);
        cards.append(&mut vec![11; 10]);
        cards.append(&mut vec![12; 10]);

        let mut new = Self { cards };
        new.shuffle();
        new
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng())
    }

    pub fn draw(&mut self) -> Result<i8, String> {
        match self.cards.pop() {
            Some(x) => Ok(x),
            None => Err("No cards in stack".to_string()),
        }
    }

    pub fn draw_playfield(&mut self) -> Result<Vec<[(bool, i8); 3]>, String> {
        if self.cards.len() < 12 {
            return Err("Not enough cards in stack".to_string());
        }
        Ok(vec![
            [
                (false, self.cards.pop().unwrap()),
                (false, self.cards.pop().unwrap()),
                (false, self.cards.pop().unwrap()),
            ],
            [
                (false, self.cards.pop().unwrap()),
                (false, self.cards.pop().unwrap()),
                (false, self.cards.pop().unwrap()),
            ],
            [
                (false, self.cards.pop().unwrap()),
                (false, self.cards.pop().unwrap()),
                (false, self.cards.pop().unwrap()),
            ],
            [
                (false, self.cards.pop().unwrap()),
                (false, self.cards.pop().unwrap()),
                (false, self.cards.pop().unwrap()),
            ],
        ])
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("Stack containing {} cards", self.cards.len()))
    }
}
