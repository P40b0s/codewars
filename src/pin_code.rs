use std::{cell::RefCell, ops::AddAssign};

use regex::Regex;



fn validate_pin(pincode: &str) -> bool
{
    let count =  pincode.chars().count();
    if count == 4 || count == 6
    {
        return pincode.chars().all(|a| a.is_digit(10) && (pincode.len() == 4 || pincode.len() == 6))
    }
    else 
    {
        return false;
    }
}
fn validate_pin_regex(pincode: &str) -> bool
{
    let p = pincode.chars().take_while(|s|s.is_numeric()).collect::<Vec<_>>();
    p.len() == 4 || p.len() == 6
}
fn validate_pin_rege(pin: &str) -> bool
{
    let count =  pin.chars().count();
    if count == 4
    {
        let re4 = Regex::new(r"[0-9]{4}").unwrap();
       
        return re4.is_match(pin)
    }
    else if count == 6
    {
        let re6 = Regex::new(r"[0-9]{6}").unwrap();
        return  re6.is_match(pin)
    }
    else
    {
        return false;
    }
}
fn validate_pin_regex_new(pin: &str) -> bool
{
    let re = Regex::new(r"^([0-9]){6}$|^([0-9]){4}$").unwrap();
   re.is_match(pin)
}

fn test_vowel(input : &str) -> usize
{
    let mut count = 0;
    for ch in input.chars()
    {
        count = count + match ch
        {
            'a' => 1,
            'e' => 1,
            'i' => 1,
            'o' => 1,
            'u' => 1,
            _ => 0
        }
    }
    count
}
//999 --> 4 (because 9*9*9 = 729, 7*2*9 = 126, 1*2*6 = 12, and finally 1*2 = 2)
fn multiply(input : u64, iteration: u64) -> u64
{
    if input > 9
    {
        let mut iter_val: Option<u64> = None;
        for n in input.to_string().chars()
        {
            let number = n.to_digit(10).unwrap() as u64;
            if iter_val.is_none()
            {
                iter_val = Some(number);
                continue;
            }
            iter_val = iter_val.and_then(|a| Some(a * number));
        }
        if iter_val.as_ref().unwrap() > &9
        {
           return multiply(iter_val.unwrap(), iteration + 1);
        }
        return iteration + 1;
    }
    else 
    {
        return iteration;
    }
}
fn multiply2(input : u64) -> u64
{
    if input > 9
    {
        let mut iter_val: Option<u64> = None;
        for n in input.to_string().chars()
        {
            let number = n.to_digit(10).unwrap() as u64;
            if iter_val.is_none()
            {
                iter_val = Some(number);
                continue;
            }
            iter_val = iter_val.and_then(|a| Some(a + number));
        }
        if iter_val.as_ref().unwrap() > &9
        {
           return multiply2(iter_val.unwrap());
        }
        return iter_val.unwrap();
    }
    else 
    {
        return input;
    }
}

fn test_walk(walker: &[char]) -> bool
{
    if walker.len() != 10
    {
        return false;
    }
    let mut xy = (0i32, 0i32);
    for ch in walker
    {
       match ch
        {
            'n' => xy.0 += 1,
            's' => xy.0 -= 1,
            'w' => xy.1 += 1,
            'e' => xy.1 -= 1,
            _ => unimplemented!()
        };
    }
    xy.0 == 0 && xy.1 == 0
}





//то же самое что у меня только норманьно на числах
pub fn persistence(num: u64) -> u64 {
    let mut n = num;
    let mut count = 0;
    while n > 9 {
        n = prod(n);
        count +=1;
    }
    count
}

fn prod(n: u64) -> u64 {
    let mut n = n;
    let mut prod = 1;
    while n > 0 {
        prod *= n%10;
        n /= 10;
    }
    prod
}


#[cfg(test)]
mod tests
{
    use std::cell::RefCell;
   
    #[test]
    fn test_high_and_low()
    {
        println!("{}", super::validate_pin("123044"));
    }
    #[test]
    fn test_high_and_low2()
    {
        println!("{}", super::validate_pin_regex("1234"));
    }

    #[test]
    fn invalid_length_tests() {
        // assert_eq!(super::validate_pin_regex("1"), false);
        // assert_eq!(super::validate_pin_regex("12"), false);
        // assert_eq!(super::validate_pin_regex("123"), false);
        // assert_eq!(super::validate_pin_regex("12345"), false);
        // assert_eq!(super::validate_pin_regex("1234567"), false);
        // assert_eq!(super::validate_pin_regex("-1234"), false);
        // assert_eq!(super::validate_pin_regex("1.234"), false);
        // assert_eq!(super::validate_pin_regex("-1.234"), false);
        assert_eq!(super::validate_pin_regex_new("1234.0"), false);
        assert_eq!(super::validate_pin_regex("00000000"), false);
    }
    #[test]
    fn test_vowel()
    {
        let t = super::test_vowel("abracadabra");
        assert_eq!(t, 5);
    }

    #[test]
    fn test_iteration()
    {
        let num = 999u64;
        //209952
        let t = super::multiply(num, 0);
        let iters = t;
        println!("число {} итераций {}", num, iters);
        //assert_eq!(iters, 3);
    }
    #[test]
    fn test_iteration2()
    {
        //493193  -->  4 + 9 + 3 + 1 + 9 + 3 = 29  -->  2 + 9 = 11  -->  1 + 1 = 2
        let num = 493193u64;
        //209952
        let t = super::multiply2(num);
        let iters = t;
        println!("итоговое число {}", iters);
        //assert_eq!(iters, 3);
    }

    #[test]
    fn test_walker()
    {
        //493193  -->  4 + 9 + 3 + 1 + 9 + 3 = 29  -->  2 + 9 = 11  -->  1 + 1 = 2
        //let num = &['n','s','n','s','n','s','n','s','n','s'];
        //209952
        //let t = super::test_walk(num);
        //println!("прогулка: {}", t);
        assert!(! super::test_walk(&['w','e','w','e','w','e','w','e','w','e','w','e']));
        //assert_eq!(iters, 3);
    }

}