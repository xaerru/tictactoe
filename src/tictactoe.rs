use rand::seq::SliceRandom;
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
};

#[derive(Debug)]
pub struct TicTacToe {
    pub board: String,
    pub arr: Vec<char>,
    pub rows: Vec<String>,
    pub turn: char,
}

pub static O: &str = r#"
▀███▀   ▀██▀ 
  ███▄  ▄█   
   ▀██▄█▀    
     ███     
   ▄█▀▀██▄   
  ▄█   ▀██▄  
▄██▄▄  ▄▄███▄
"#;

pub static X: &str = r#"
 ▄▄█▀▀██▄  
▄██▀    ▀██▄
██▀      ▀██
██        ██
██▄      ▄██
▀██▄    ▄██▀
  ▀▀████▀▀  
"#;

pub static HASH: &str = r#"
    ▄█▀ ██ 
    ██ ▄█▀ 
 █████████ 
   ██ ██   
██████████ 
 ▄█▀ ██    
 ██ ▄█▀    
"#;

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            board: String::new(),
            arr: vec!['#'; 10],
            rows: vec!["".to_string(); 3],
            turn: *['X', 'O'].choose(&mut rand::thread_rng()).unwrap(),
        }
    }

    pub fn get(&mut self, num: usize) -> Vec<Spans> {
        let o: Vec<Spans> = O
            .lines()
            .map(|f| {
                Spans::from(Span::styled(
                    f,
                    Style::default()
                        .fg(Color::Rgb(191, 97, 106))
                        .add_modifier(Modifier::BOLD),
                ))
            })
            .collect();
        let x: Vec<Spans> = X
            .lines()
            .map(|f| {
                Spans::from(Span::styled(
                    f,
                    Style::default()
                        .fg(Color::Rgb(143, 188, 187))
                        .add_modifier(Modifier::BOLD),
                ))
            })
            .collect();
        let hash: Vec<Spans> = HASH
            .lines()
            .map(|f| {
                Spans::from(Span::styled(
                    f,
                    Style::default()
                        .fg(Color::Rgb(163, 190, 140))
                        .add_modifier(Modifier::BOLD),
                ))
            })
            .collect();
        let val = self.arr[num];
        if val == 'X' {
            x
        } else if val == 'O' {
            o
        } else {
            hash
        }
    }

    pub fn play(&mut self, num: i32) {
        if self.arr[num as usize] == '#' {
            if self.turn == 'O' {
                self.arr[num as usize] = 'O';
                self.turn = 'X'
            } else if self.turn == 'X' {
                self.arr[num as usize] = 'X';
                self.turn = 'O';
            }
        }
    }

    pub fn reset(&mut self) {
        self.arr = vec!['#'; 10];
    }

    pub fn turn(&mut self) -> Vec<Spans> {
        let o: Vec<Spans> = O
            .lines()
            .map(|f| {
                Spans::from(Span::styled(
                    f,
                    Style::default()
                        .fg(Color::Rgb(191, 97, 106))
                        .add_modifier(Modifier::BOLD),
                ))
            })
            .collect();
        let x: Vec<Spans> = X
            .lines()
            .map(|f| {
                Spans::from(Span::styled(
                    f,
                    Style::default()
                        .fg(Color::Rgb(143, 188, 187))
                        .add_modifier(Modifier::BOLD),
                ))
            })
            .collect();
        if self.turn == 'X' {
            x
        } else {
            o
        }
    }

    pub fn is_full(&mut self) -> bool {
        let arr = &self.arr[1..];
        if arr.contains(&'#') {
            false
        } else {
            true
        }
    }

    pub fn winner(&mut self) -> Option<String> {
        let arr = &self.arr;
        let checker =
            |a: usize, b: usize, c: usize| arr[a] != '#' && arr[a] == arr[b] && arr[b] == arr[c];
        if checker(1, 2, 3)
            || checker(4, 5, 6)
            || checker(7, 8, 9)
            || checker(7, 5, 3)
            || checker(9, 5, 1)
            || checker(7, 4, 1)
            || checker(8, 5, 2)
            || checker(9, 6, 3)
        {
            Some(if self.turn == 'X' {
                'X'.to_string()
            } else {
                'O'.to_string()
            })
        } else {
            None
        }
    }
}
