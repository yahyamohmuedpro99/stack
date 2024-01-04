

// first set size for the stack
fn new_stack(max_size: usize)->Vec<u32> {
    let vec:Vec<u32> = Vec::with_capacity(max_size);
    vec
}

// pop the elements but should check if there are elements or not 
fn pop_elements(stack:&mut Vec<u32>) ->Option<Vec<u32>> {
    let mut pop_val:Option<u32> = stack.pop();
    println!("pop_element {:?}",pop_val);
    pop_val 
}
// push element but check if it exceeds the max size
fn push_elements(stack:&mut Vec<u32>,item:u32,max_size:usize) -> Option<Vec<u32>> {
    if stack.len()==max_size{
        println!("can't push more than{}",stack.len());
    }else{
        stack.push(item);
        stack
    }
}
fn main() {
    println!("Hello, world!");
}
