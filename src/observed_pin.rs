use std::{collections::HashMap, fmt::{format, Display}, default};
use once_cell::sync::Lazy;
use rayon::prelude::IndexedParallelIterator;
pub static SCHEME: Lazy<HashMap<char, Vec<u8>>> = Lazy::new(|| HashMap::from(
    [
        ('1', vec![1, 2, 4]),
        ('2', vec![1, 2, 3, 5]),
        ('3', vec![2, 3, 6]),
        ('4', vec![1, 4, 5, 7]),
        ('5', vec![2, 4, 5, 6, 8]),
        ('6', vec![3, 5, 6, 9]),
        ('7', vec![4, 7, 8]),
        ('8', vec![0, 5, 7, 8, 9]),
        ('9', vec![6, 8, 9]),
        ('0', vec![0, 8]),
    ]
));

//2,3,6
//3,5,6,9
//6,8,9

//"Pack my box with five dozen liquor jugs."
fn pin(observed: &str) -> bool
{
    
    //завичсит от длинны пина
   
    let mut contains : Vec<String> = vec![];
    let pin_len = observed.len();
    for obs in observed.chars()
    {
        
    }
    // let rnd = range.filter(|f|
    // {
    //     let f = f.to_string();
        
    //     f.contains("1") && f.len() == pin_len
    // }).collect::<Vec<i32>>();
    return false;

}

fn search_range(contains: &Vec<u8>, pin_len: usize)
{
    let range = 0..9999;
    let rnd = range.filter(|f|
    {
        let f = f.to_string();
        
        f.contains("1") && f.len() == pin_len
    }).collect::<Vec<i32>>();
}

fn get_pins(observed: &str) -> Vec<String>
{
    let items = observed.chars().map(|m| SCHEME.get(&m).unwrap().clone()).collect::<Vec<Vec<u8>>>();
    let mut res:  Vec<String> = vec![];
    let mut compl = false;
    let mut x_indeces : HashMap<usize, usize> = HashMap::new();
    for i in 0..items.len()
    {
        x_indeces.insert(i, 0);
    }
    while !compl
    {
        let mut subres: String = String::new();
        for (i, item) in items.iter().enumerate()
        {
            let itm = item[*x_indeces.get(&i).unwrap()];
            subres.push(char::from_digit(itm as u32, 10).unwrap());
            if i == items.len() - 1
            {
                let mi = x_indeces.get_mut(&i).unwrap();
                *mi += 1;
            }

            if x_indeces.get(&i).unwrap() >= &item.len()
            {
                let rev = (0..=i).into_iter().rev();
                for r in rev
                {
                    if x_indeces.get(&r).unwrap() >= &items.iter().nth(r).as_ref().unwrap().len()
                    {
                        if r == 0
                        {
                            compl = true;
                            break;
                        }
                        let next_index = x_indeces.get_mut(&r).unwrap();
                        *next_index = 0;
                        let self_index = x_indeces.get_mut(&(r-1)).unwrap();
                        *self_index += 1;
                    }
                    else 
                    {
                        break;
                    }
                }
            }
        }
        res.push(subres);
    }
    return res;
}

//на первом месте этот код
use itertools::Itertools;

fn get_pins_fp(observed: &str) -> Vec<String> {
    let possibilities = HashMap::from([
        ('0', vec!['0','8']),
        ('1', vec!['1','2','4']),
        ('2', vec!['1','2','3','5']),
        ('3', vec!['2','3','6']),
        ('4', vec!['1','4','5','7']),
        ('5', vec!['2','4','5','6','8']),
        ('6', vec!['3','5','6','9']),
        ('7', vec!['4','7','8']),
        ('8', vec!['0','5','7','8','9']),
        ('9', vec!['6','8','9']),
    ]);
    
    observed.chars()
        .map(|c| possibilities.get(&c).unwrap())
        .multi_cartesian_product()
        .map(|v| v.into_iter().collect())
        .collect()
}

#[cfg(test)]
mod tests
{
    use std::collections::HashMap;
    use itertools::Itertools;
    use crate::observed_pin::SCHEME;
   
    #[test]
    fn test_range()
    {
        let mut vectors : Vec<String> = vec![];
        let num = 369.to_string();
        let mut vecs = num.chars().map(|m| SCHEME.get(&m).unwrap().clone()).collect::<Vec<Vec<u8>>>();
        println!("{:?}", vecs);
        // let mut last = vecs.last_mut().unwrap();
        // let mut modifers: Vec<(usize, usize)> = vec![];
        // // if vecs.len() == 1
        // // {
        // //     let v = first.iter().map(|m| m.to_string()).collect::<Vec<String>>();
        // // }
        // for (i, len) in vecs.iter().enumerate()
        // {
        //     if !modifers.iter().any(|f| f.0 == i)
        //     {
        //         modifers.push((i, 0));
        //     }
        //     for v in vecs
        //     {
                
        //     }
        // }
    }

    // fn vec_sum(vec_1: Vec<u8>, vec_2 : Vec<u8>) -> Vec<String>
    // {
    //     let mut vectors : Vec<String> = vec![];
    //     for v1 in vec_1
    //     {
    //         for v2 in vec_2
    //         {
    //             vectors.push([v1.to_string(), v2.to_string()].concat());
    //         }
    //     }
    //     vectors
    // }
       // let vecs = num.chars().map(|m| *SCHEME.get(&m).unwrap()).collect::<Vec<Vec<u8>>>();
    
    #[test]
    fn sample_tests() {
        assert_eq!(super::get_pins("8").iter().sorted().collect::<Vec<&String>>(), 
            vec!["0", "5", "7", "8", "9"]);
        assert_eq!(super::get_pins("11").iter().sorted().collect::<Vec<&String>>(), 
            vec!["11", "12", "14",  "21", "22", "24",  "41", "42", "44"]);
        assert_eq!(super::get_pins("369").iter().sorted().collect::<Vec<&String>>(), 
            vec!["236", "238", "239",  "256", "258", "259",  "266", "268", "269",  "296", "298", "299", 
                "336", "338", "339",  "356", "358", "359",  "366", "368", "369",  "396", "398", "399", 
                "636", "638", "639",  "656", "658", "659",  "666", "668", "669",  "696", "698", "699"]);
    }
}