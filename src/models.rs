use std::ops::Index;
use std::ops::IndexMut;

use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub nickname: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    pub(crate) scores: Vec<ScoreWithPlayer>,
    pub(crate) id: i32,
    // date: Date,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ScoreWithPlayer {
    pub(crate) ace: Option<i32>,
    pub(crate) two: Option<i32>,
    pub(crate) three: Option<i32>,
    pub(crate) four: Option<i32>,
    pub(crate) five: Option<i32>,
    pub(crate) six: Option<i32>,
    pub(crate) seven: Option<i32>,
    pub(crate) eight: Option<i32>,
    pub(crate) nine: Option<i32>,
    pub(crate) ten: Option<i32>,
    pub(crate) jack: Option<i32>,
    pub(crate) queen: Option<i32>,
    pub(crate) king: Option<i32>,
    pub(crate) player_id: i32,
    pub(crate) player_name: String,
}

// impl ScoreWithPlayer {
//     pub(crate) fn score(&self) -> Score {
//         Score {
//             ace: self.ace,
//             two: self.two,
//             three: self.three,
//             four: self.four,
//             five: self.five,
//             six: self.six,
//             seven: self.seven,
//             eight: self.eight,
//             nine: self.nine,
//             ten: self.ten,
//             jack: self.jack,
//             queen: self.queen,
//             king: self.king,
//         }
//     }

//     pub(crate) fn player(&self) -> Player {
//         Player {
//             id: self.player_id,
//             name: self.player_name.clone(),
//             nickname: None,
//         }
//     }
// }

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Score {
    pub(crate) ace: Option<i32>,
    pub(crate) two: Option<i32>,
    pub(crate) three: Option<i32>,
    pub(crate) four: Option<i32>,
    pub(crate) five: Option<i32>,
    pub(crate) six: Option<i32>,
    pub(crate) seven: Option<i32>,
    pub(crate) eight: Option<i32>,
    pub(crate) nine: Option<i32>,
    pub(crate) ten: Option<i32>,
    pub(crate) jack: Option<i32>,
    pub(crate) queen: Option<i32>,
    pub(crate) king: Option<i32>,
}

impl Index<usize> for Score {
    type Output = Option<i32>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.ace,
            1 => &self.two,
            2 => &self.three,
            3 => &self.four,
            4 => &self.five,
            5 => &self.six,
            6 => &self.seven,
            7 => &self.eight,
            8 => &self.nine,
            9 => &self.ten,
            10 => &self.jack,
            11 => &self.queen,
            12 => &self.king,
            _ => panic!("invalid card index: {index}"),
        }
    }
}

impl IndexMut<usize> for Score {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.ace,
            1 => &mut self.two,
            2 => &mut self.three,
            3 => &mut self.four,
            4 => &mut self.five,
            5 => &mut self.six,
            6 => &mut self.seven,
            7 => &mut self.eight,
            8 => &mut self.nine,
            9 => &mut self.ten,
            10 => &mut self.jack,
            11 => &mut self.queen,
            12 => &mut self.king,
            _ => panic!("invalid card index: {index}"),
        }
    }
}
