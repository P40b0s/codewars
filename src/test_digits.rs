

fn test_digits(input: i32) -> i32
{
    if input.is_negative()
    {
        return 0;
    }
    let mut sum = 0;
    for i in 1..input
    {
        let three = test_number(i as f64, 3f64);
        let five = test_number(i as f64, 5f64);
        if three == five
        {
            sum = sum + three;
        }
        else 
        {
            sum = sum + three + five;
        }
    }
    sum
}

fn test_number(number: f64, mult: f64) -> i32
{
    let res = number / mult;
    let floor = res.floor();
    let part = floor - res;
    if part == 0 as f64
    {
        println!("{}/{}={} return {}", number, mult, res, number);
        return number as i32;
    }
    else 
    {
        return 0;
    }
}
#[cfg(test)]
mod tests
{   
    #[test]
    fn test_num()
    {
        let one  = 10;
        let two = 3;

        let res = one as f64  / two as f64;
        if res.is_sign_negative()
        {
            println!("{}", 0);
        }
        else
        {
            let main = res.floor();
            let result = main - res;
            if result == 0 as f64
            {
                println!("{}", res);
            }
        }
    }
    #[test]
    fn test_diigits()
    {
        println!("{}", super::test_digits(-123));
    }
}