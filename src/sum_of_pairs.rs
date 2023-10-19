use std::{sync::{Arc, Mutex, mpsc::channel}, thread};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};



fn sum(ints: &[i8], s: i8) -> Option<(i8, i8)> 
{
    let mut results: Vec<(usize, (i8, i8))> = vec![];
    'outer: for (i, num) in ints.iter().enumerate()
    {
        for ind in (i+1)..ints.len()
        {
            if results.iter().any(|a| a.0 < ind)
            {
                break 'outer;
            }
            if num + ints[ind] == s
            {
                results.push((ind, (*num, ints[ind])));
            }
        }
    }
    results.sort_by_key(|s|s.0);
    results.first().and_then(|a| Some(a.1))
}

fn sum2(ints: &[i8], s: i8) -> Option<(i8, i8)> 
{
    let mut results: Vec<(usize, (i8, i8))> = vec![];
    let br = Arc::new(Mutex::new(false));
    let (tx, rx) = channel::<(usize, (i8, i8))>();
    for (i, num) in ints.iter().enumerate()
    {
        if *br.lock().unwrap() == true
        {
            break;
        }
        let br = Arc::clone(&br);
        let tx_cloned = tx.clone();
        thread::scope(|sc|
        {
            let results = results.iter().map(|m| m.0).collect::<Vec<usize>>();
            let th = sc.spawn(move ||
            {
                for ind in (i+1)..ints.len()
                {
                    if results.iter().any(|a| a < &ind)
                    {
                        let mut b = br.lock().unwrap();
                        *b = true;
                        break;
                    }
                    if num + ints[ind] == s
                    {
                        tx_cloned.send((ind, (*num, ints[ind]))).unwrap();
                    }
                }
            });
            th.join().unwrap();
        });
    }
    std::mem::drop(tx);
    while let Ok(r) = rx.try_recv()
    {
        results.push(r);
    }
    results.sort_by_key(|s|s.0);
    results.first().and_then(|a| Some(a.1))
}



#[cfg(test)]
mod tests
{
   
    #[test]
    fn test()
    {
        let l1 = [1, 4, 8, 7, 3, 15];
        let l2 = [1, -2, 3, 0, -6, 1];
        let l3 = [20, -13, 40];
        let l4 = [1, 2, 3, 4, 1, 0];
        let l5 = [10, 5, 2, 3, 7, 5];
        let l6 = [4, -2, 3, 3, 4];
        let l7 = [0, 2, 0];
        let l8 = [5, 9, 13, -3];
        assert_eq!(super::sum2(&l1, 8), Some((1, 7)));
        assert_eq!(super::sum2(&l2, -6), Some((0, -6)));
        assert_eq!(super::sum2(&l3, -7), None);
        assert_eq!(super::sum2(&l4, 2), Some((1, 1)));
        assert_eq!(super::sum2(&l5, 10), Some((3, 7)));
        assert_eq!(super::sum2(&l6, 8), Some((4, 4)));
        assert_eq!(super::sum2(&l7, 0), Some((0, 0)));
        assert_eq!(super::sum2(&l8, 10), Some((13, -3)));
        //let res = super::sum(&l2, 10);
        //println!("{:?}", res.unwrap());
      
    }
}