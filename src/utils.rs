use std::str::FromStr;
use std::fmt::Debug;
pub fn input<T>() -> T 
where 
    T:FromStr,
    T::Err: Debug ,
{
    let mut input=String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("faild to read input");
    input.trim().parse().expect("invalid input")
}