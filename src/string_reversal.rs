use crate::stack::stack::Stack;
use crate::utils::input;


pub fn string_reversal(){
    println!("enter string you want to reverse : ");
    let orginal_string:String=input();
    let mut string_stack = Stack::new(orginal_string.len());
    let mut reversed_string=String::with_capacity(orginal_string.len()); 
    for i in orginal_string.chars(){
        string_stack.push(i,orginal_string.len());
    }
    while let Some(i)=string_stack.pop() {
        reversed_string.push(i)
    }

    println!("you enter '{}' and the reversed is '{}'", orginal_string,reversed_string);
}
