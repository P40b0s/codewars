use std::{sync::{Arc, Mutex, mpsc::channel}, thread, time::Duration, ops::{Add, Mul}};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};


fn sudoku(puzzle: &mut [[u8; 9]; 9]) 
{
    
}

fn get_vert_slice(puzzle: &[[u8; 9]; 9], source_pos_y: usize) -> Vec<u8>
{
    let mut res : Vec<u8> = vec![];
    for horizontal in puzzle.iter().enumerate()
    {
        for vertical in horizontal.1.iter().enumerate()
        {
            if vertical.0 == source_pos_y
            {
                res.push(*vertical.1);
            }
        }
    }
    res
}
fn get_hor_slice(puzzle: &[[u8; 9]; 9], source_pos_x: usize) -> Vec<u8>
{
    let mut res : Vec<u8> = vec![];
    for horizontal in puzzle.iter().enumerate()
    {
        if horizontal.0 == source_pos_x
        {
            res = horizontal.1.to_vec();
        }
    }
    res
}

fn get_block(puzzle: &[[u8; 9]; 9], source_pos_x: usize)-> Vec<u8>
{
    
}



#[cfg(test)]
mod tests
{

    #[test]
    fn vertical_searcher_1() 
    {
        let mut puzzle = [
                [6, 0, 5, 7, 2, 0, 0, 3, 9],
                [4, 0, 0, 0, 0, 5, 1, 0, 0],
                [0, 2, 0, 1, 0, 0, 0, 0, 4],
                [0, 9, 0, 0, 3, 0, 7, 0, 6],
                [1, 0, 0, 8, 0, 9, 0, 0, 5],
                [2, 0, 4, 0, 5, 0, 0, 8, 0],
                [8, 0, 0, 0, 0, 3, 0, 2, 0],
                [0, 0, 2, 9, 0, 0, 0, 0, 1],
                [3, 5, 0, 0, 6, 7, 4, 0, 8],
            ];
        
        for h in puzzle.iter().enumerate()
        {
            for v in h.1.iter().enumerate()
            {
                if v.1 == &5
                {
                    println!("найденная строка: {:?}", super::get_vert_slice(&puzzle, v.0));
                }
                
            }
        }
    
    }

    #[test]
    fn puzzle_1() 
    {
        let mut puzzle = [
                [6, 0, 5, 7, 2, 0, 0, 3, 9],
                [4, 0, 0, 0, 0, 5, 1, 0, 0],
                [0, 2, 0, 1, 0, 0, 0, 0, 4],
                [0, 9, 0, 0, 3, 0, 7, 0, 6],
                [1, 0, 0, 8, 0, 9, 0, 0, 5],
                [2, 0, 4, 0, 5, 0, 0, 8, 0],
                [8, 0, 0, 0, 0, 3, 0, 2, 0],
                [0, 0, 2, 9, 0, 0, 0, 0, 1],
                [3, 5, 0, 0, 6, 7, 4, 0, 8],
            ];
        let solution = [
                [6, 1, 5, 7, 2, 4, 8, 3, 9],
                [4, 8, 7, 3, 9, 5, 1, 6, 2],
                [9, 2, 3, 1, 8, 6, 5, 7, 4],
                [5, 9, 8, 4, 3, 2, 7, 1, 6],
                [1, 3, 6, 8, 7, 9, 2, 4, 5],
                [2, 7, 4, 6, 5, 1, 9, 8, 3],
                [8, 4, 9, 5, 1, 3, 6, 2, 7],
                [7, 6, 2, 9, 4, 8, 3, 5, 1],
                [3, 5, 1, 2, 6, 7, 4, 9, 8],
            ];
        
        super::sudoku(&mut puzzle);
        assert_eq!(puzzle, solution, "\nYour solution (left) did not match the correct solution (right)");
    }
    
    #[test]
    fn puzzle_2() 
    {
        let mut puzzle = [
                [0, 0, 8, 0, 3, 0, 5, 4, 0],
                [3, 0, 0, 4, 0, 7, 9, 0, 0],
                [4, 1, 0, 0, 0, 8, 0, 0, 2],
                [0, 4, 3, 5, 0, 2, 0, 6, 0],
                [5, 0, 0, 0, 0, 0, 0, 0, 8],
                [0, 6, 0, 3, 0, 9, 4, 1, 0],
                [1, 0, 0, 8, 0, 0, 0, 2, 7],
                [0, 0, 5, 6, 0, 3, 0, 0, 4],
                [0, 2, 9, 0, 7, 0, 8, 0, 0],
            ];
        let solution = [
                [9, 7, 8, 2, 3, 1, 5, 4, 6],
                [3, 5, 2, 4, 6, 7, 9, 8, 1],
                [4, 1, 6, 9, 5, 8, 3, 7, 2],
                [8, 4, 3, 5, 1, 2, 7, 6, 9],
                [5, 9, 1, 7, 4, 6, 2, 3, 8],
                [2, 6, 7, 3, 8, 9, 4, 1, 5],
                [1, 3, 4, 8, 9, 5, 6, 2, 7],
                [7, 8, 5, 6, 2, 3, 1, 9, 4],
                [6, 2, 9, 1, 7, 4, 8, 5, 3],
            ];
        
        super::sudoku(&mut puzzle);
        assert_eq!(puzzle, solution, "\nYour solution (left) did not match the correct solution (right)");
    }
    
}