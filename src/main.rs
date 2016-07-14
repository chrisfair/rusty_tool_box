use std::env;

fn main() {

   let list_of_arguments:Vec<String> = env::args().collect();
   let ref first_argument = list_of_arguments[0];
   println!("{}", first_argument);

}

