use std::{sync::{Arc, Mutex, mpsc::channel}, thread, time::Duration, ops::{Add, Mul}};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};


use std::time::Instant;

// fn snail(matrix: &[Vec<i32>]) -> Vec<i32> 
// {
//     if matrix.len() < 3
//     {
//         let mut v: Vec<i32> = vec![];
//         matrix.iter().for_each(|f| v.extend(f.iter()));
//         return v;
//     }
//     else
//     {
//         let mut result: Vec<i32> = vec![];
//         let mut south = matrix.len() - 1;
//         for (i, line) in matrix.iter().enumerate()
//         {
//             let modifer = i + 1;
//             //if next line is downline
//             if i == south
//             {
//                 //let right_slice = get_vertical_line(matrix, modifer, south -1, right_index);
//                 break;
//             }
//             if i == 0
//             {
//                 result.extend(line);
//             }
//             else
//             {
//                 result.extend(&line[0..line.len() - modifer]);
//             }
           
//             let right_index = line.len() - modifer;
//             let left_modifer = i;
//             let right_slice = get_vertical_line(matrix, modifer, south -1, right_index);
//             result.extend(right_slice);
//             let mut south_slice = Vec::<i32>::with_capacity(matrix[south].len());
//             for ss in &matrix[south]
//             {
//                 south_slice.insert(0, *ss);
//             }
//             result.extend(south_slice);
//             let left_slice = get_vertical_line(matrix, modifer, south -1, left_modifer);
//             result.extend(left_slice);
//             south-=1;
//         }
//         return vec![];
//     } 
// }

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> 
{
    if matrix.len() <=1
    {
        let mut v: Vec<i32> = vec![];
        matrix.iter().for_each(|f| v.extend(f.iter()));
        return v;
    }
    else
    {
        let mut result: Vec<i32> = vec![];
        result = nail(matrix.to_vec(), vec![]);
        result
    } 
}

fn nail(matrix: Vec<Vec<i32>>, output: Vec<i32>) -> Vec<i32>
{
    let mut output = output;
    if matrix[0].len() == 1
    {
        output.extend(&matrix[0]);
        return  output;
    }
    else if matrix[0].len() == 2
    {
        output.extend(vec![matrix[0][0], matrix[0][1], matrix[1][1], matrix[1][0]]);
        return output;
    }
    else
    {
        let mut new_matrix = vec![];
        let up = &matrix[0];
        output.extend(up);
        let right = get_vertical_line(&matrix, 1, matrix.len() -2, up.len() -1);
        output.extend(right);
        let mut south_slice = Vec::<i32>::with_capacity(matrix[matrix.len() -1].len());
        for ss in &matrix[matrix.len() -1]
        {
            south_slice.insert(0, *ss);
        }
        output.extend(south_slice);
        let left = get_vertical_line(&matrix, 1, matrix.len() -2, 0);
        let mut left_reverse = Vec::<i32>::with_capacity(left.len());
        for ss in left
        {
            left_reverse.insert(0, ss);
        }
        output.extend(left_reverse);
        for (i, m) in matrix.iter().enumerate()
        {
            if i != 0 && i != matrix.len() -1
            {
                
                new_matrix.push(m[1..=m.len() -2].to_vec());
            }
        }
        return nail(new_matrix, output);
    }
    
}

fn get_vertical_line(matrix: &[Vec<i32>], up: usize, down: usize, line: usize) -> Vec<i32>
{
    let mut v : Vec<i32> = vec![];
    for i in &matrix[up..=down]
    {
        v.push(i[line]);
    }
    v
}

#[cfg(test)]
mod tests
{

   
    #[test]
    fn test_plus()
    {
        let square = &[
            vec![1,2,3],
            vec![8,9,4],
            vec![7,6,5],
        ];
        let expected = vec![1,2,3,4,5,6,7,8,9];
        assert_eq!(super::snail(square), expected);
    }
    #[test]
    fn test_plus3()
    {
        let square = &[
            vec![1,2],
            vec![4,3],
        ];
        let expected = vec![1,2,3,4];
        assert_eq!(super::snail(square), expected);
    }
    #[test]
    fn test_plus4()
    {
        let square = &[
            vec![1,2,3,1],
            vec![4,5,6,4],
            vec![7,8,9,7],
            vec![7,8,9,7],
        ];
        let expected = vec![1,2,3,1,4,7,7,9,8,7,7,4,5,6,9,8];
        assert_eq!(super::snail(square), expected);
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(super::snail(square), expected);
    }
   
      
    
    #[test]
    fn test_ost()
    {
        println!("{}", (65/10)%10);

        
      
    }
}