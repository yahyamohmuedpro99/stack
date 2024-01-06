use colored::*;
use crate::{utils::input, stack::stack::Stack};
pub fn numbers(){
    
    println!("if you want to create a stack afford the size of the stack ");
    let size_stack: usize = input();
    let mut stack = Stack::new(size_stack as usize);

    loop {
        println!(
            "\n\n{}\n",
            " ----------Menu ---------- "
                .red()
                .bold()
                .bright_yellow()
                .blink()
        );
        println!(
            "{} {} {} {} {}",
            "  1-push \n".green().bold().on_black(),
            " 2-pop  \n".yellow().bold().on_black(),
            " 3-Display \n".blue().bold().on_black(),
            " 4-size \n".magenta().bold().on_black(),
            " 5-exit".red().bold().on_black()
        );
        println!("enter one of methods : ");

        let choice = input();
        match choice {
            1 => {
                println!("enter the value to insert");
                let element:usize = input();
                Stack::push(&mut stack, element, size_stack as usize);
            }
            2 => println!("you poped {:?} from the stack", Stack::pop(&mut stack)),
            3 => println!(
                "The stack now is {:?}",
                &stack.data
            ),
            4 => println!("the size of the stack now is {:?}", Stack::size(&stack)),
            5 => {
                println!("see u latter :)");
                break;
            }
            _ => println!("wrong choice"),
        }
    }


}