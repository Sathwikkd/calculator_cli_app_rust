
use::std::io;

fn calculator(num1:f64,num2:f64,op:char){
    match op {
        '+' =>{
            print!("Result = {}",num1+num2);
        },
        '-' =>{
            print!("Result = {}",num1-num2);
        },
        '*' =>{
            print!("Result = {}",num1*num2);
        },
        '/' => {
             if num2 == 0_f64 {
                print!("Result = Infinity (∞)");
             }else{
                print!("Result = {}",num1/num2);
             }
           
        },
        
     op => {
        print!("{} is invalid opeartor!!  😞",op);
     }
    

    }
}
fn main(){

    loop {
        println!("\n🦀🦀 🦀🦀🦀 🦀🦀🦀🦀🦀 🦀🦀🦀 🦀🦀\n");

        println!("Welcome to my simple calculator built using RUST");
        let mut num1=String::new();
        let mut num2=String::new();
        let mut op=String::new();

        println!("Enter the number 1:");
        io::stdin().read_line(&mut num1).expect("Failed to read num1  😞");
        println!("Enter the number 2:");
        io::stdin().read_line(&mut num2).expect("failed to read num 2  😞");
        println!("Enter the operator to be perform");
        println!("(+,-,*,/)");
        io::stdin().read_line(&mut op).expect("failed to read option  😞");

        let num1:f64=num1.trim().parse().expect("Failed to parse num 1  😞");
        let num2:f64=num2.trim().parse().expect("failed to parse num 2  😞");
        let op:char=op.trim().parse().expect("unable to parse the operator  😞");

        calculator(num1, num2, op);
    }

}