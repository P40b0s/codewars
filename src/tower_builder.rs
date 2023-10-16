fn test_build_floors(floors: usize) -> Vec<String>
{
    let mut floors_vec: Vec<String> = Vec::with_capacity(floors);
    for f in 1..=floors
    {
        floors_vec.push(build_floor(floors, f));
    }
    floors_vec
}

fn get_stars_count(floor: usize) -> usize
{
    (floor * 2) - 1
}

fn build_floor(floors_count: usize, current_floor : usize) -> String
{
    let stars = get_stars_count(current_floor);
    let max_stars = get_stars_count(floors_count);
    let ws = (max_stars - stars) /2;
    [" ".repeat(ws), "*".repeat(stars), " ".repeat(ws)].concat()
}

#[cfg(test)]
mod tests
{   
    #[test]
    fn test_builder()
    {
      let f = super::test_build_floors(6);
      for v in f
      {
        println!("{}", &v)
      }
    }
}