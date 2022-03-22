use std::io;



fn menu() {

    let b1 = 0;

    let mut v = vec!["Potatoes", "Soda", "Juice", "Bread"];

    while b1 < 4 {
        
        println!(" ");
        println!("1. Show cart");
        println!("2. Add to cart");
        println!("3. Delete item");

        let mut line = String::new();
        println!(" ");
        println!("Enter a number in the cart: ");

        io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

        let line: u32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        

        if line == 1 {
            for i in &v {
                println!("{}", i);
            }
        } else if line == 2 {
            let mut item = String::new();
            println!("Enter a number in the cart: ");
    
            io::stdin()
            .read_line(&mut item)
            .expect("Failed to read line");


            v.push("Bird");

            for i in &v {
                println!("{}", i);
            }
        } else if  line == 3 {
            for i in &v {
                println!("{}", i);
            }
            let mut delete = String::new();
            println!("Pick which item and type the number: ");
    
            io::stdin()
            .read_line(&mut delete)
            .expect("Failed to read line");
    
            let delete: u32 = match delete.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // v.remove(delete);



        } else {
            break;
        }
        
    } 

    
}

fn main() {
    menu();
}
