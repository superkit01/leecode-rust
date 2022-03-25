pub struct Solution{}

/**
 *          vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9'];
 */
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
            let mut horization:HashMap<i32,HashSet<char>>=HashMap::new();
            let mut vertical:HashMap<i32,HashSet<char>>=HashMap::new();
            let mut cube:HashMap<i32,HashSet<char>>=HashMap::new();

            for i in 0..board.len(){
                for j in 0..board.len(){
                    if board[i][j]=='.'{
                        continue;
                    }
                    let hset=horization.entry(i as i32).or_insert(HashSet::new());
                    if !hset.insert(board[i][j]){
                       return false;
                    }

                    let vset=vertical.entry(j as i32).or_insert(HashSet::new());
                    if !vset.insert(board[i][j]){
                       return false;
                    }

                    let cset=cube.entry(((i/3)*3+j/3) as i32).or_insert(HashSet::new());
                    if !cset.insert(board[i][j]){
                       return false;
                    }
                }
            }
            return true;


    }
}