use std::{sync::{Arc, Mutex, mpsc::channel}, thread, time::Duration};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};



fn multiply(a: &str, b: &str) -> String 
{
    
    let mut one = a;
    if a.len() > 1
    {
        one = a.trim_start_matches("0");
    }
    let mut two = b.trim_start_matches("0");
    if b.len() > 1
    {
        two = b.trim_start_matches("0");
    }
    18446744073709551615


}

fn split(s: &str)
{
    let mut arr : Vec<String> = vec![];
    if s.contains("0")
    {
        let zeros = s.split("0").collect::<Vec<&str>>();
        for z in zeros
        {
            if zeros.len() > 20
            {
                
            }
        }
    }
    else
    {

    }
}

use std::time::Instant;



#[cfg(test)]
mod tests
{
   
    #[test]
    fn test()
    {
        //let res = super::sum(&l2, 10);
        //println!("{:?}", res.unwrap());
      
    }
}