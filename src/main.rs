use std::string;

fn main(){
    let mut hop = "ho".to_string();
    hop.push_str("govno");
    println!("{}",hop);
    let operation = "ru";
    match operation{
        "pl"=>print!("Результат сложения {}",plus(1,1)),
        "ru"=>print!("Результат вычитания {}",ruz(1900000,11189)),
        "su"=>print!("Результат умножения {}",sum(1,1)),
        "del"=>print!("Результат деления {}",del(1,1)),
      other =>println!("Такой операции нет")
    }
}
fn plus(num1:i32,num2:i32)->i32{
     num1  +num2
}fn ruz(num1:i32,num2:i32)->i32{
     num1  -num2
}fn sum(num1:i32,num2:i32)->i32{
     num1  *num2
}fn del(num1:i32,num2:i32)->f64{
    num1 as f64  /num2 as f64
}