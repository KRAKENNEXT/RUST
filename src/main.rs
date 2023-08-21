// use std::string;
//
// fn main(){
//     let operation = "ru";
//     match operation{
//         "pl"=>print!("Результат сложения {}",plus(1,1)),
//         "ru"=>print!("Результат вычитания {}",ruz(1900000,11189)),
//         "su"=>print!("Результат умножения {}",sum(1,1)),
//         "del"=>print!("Результат деления {}",del(1,1)),
//       other =>println!("Такой операции нет")
//     }
// }
// fn plus(num1:i32,num2:i32)->i32{
//      num1  +num2
// }
// fn ruz(num1:i32,num2:i32)->i32{
//      num1  -num2
// }
// fn sum(num1:i32,num2:i32)->i32{
//      num1  *num2
// }
// fn del(num1:i32,num2:i32)->f64{
//     num1 as f64  /num2 as f64
// }
//
//





use std::fmt::format;
fn main(){
    let user_input ="ne";
    match user_input{
        "gg"=>println!("Ваш выбор: {}",roulette1(" Zipa".to_string())),
        "ff"=>println!("Ваш выбор: {}",roulette2(" Zipae".to_string())),
        "lao"=>println!("Ваш выбор: {}",roulette3(" Zipar".to_string())),
        "mar"=>println!("Ваш выбор: {}",roulette4(" Zipat".to_string())),
        "mal"=>println!("Ваш выбор: {}",roulette5(" Zipay".to_string())),
        "ne"=>println!("Ваш выбор: {}",roulette6(" Zipau".to_string())),
        "ta"=>println!("Ваш выбор: {}",roulette7(" Zipai".to_string())),
        "bb"=>println!("Ваш выбор: {}",roulette8(" Zipao".to_string())),
        "fi"=>println!("Ваш выбор: {}",roulette9(" Zipap".to_string())),
        "nem"=>println!("Ваш выбор: {}",roulette10(" Zipap".to_string())),
        "bas"=>println!("Ваш выбор: {}",roulette11(" Zipas".to_string())),
        "kat"=>println!("Ваш выбор: {}",roulette12(" Zipad".to_string())),
        "hor"=>println!("Ваш выбор: {}",roulette13(" Zipaf".to_string())),
        "bret"=>println!("Ваш выбор: {}",roulette14(" Zipag".to_string())),
        "dat"=>println!("Ваш выбор: {}",roulette15(" china".to_string())),
        other=>println!("Такого параметра не существует")
    };
}


fn roulette1(name:String) -> String {
    let gg = "Hello".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {} {} ",gg,name);
    return none;
}


fn roulette2(name:String)->String{
    let ff ="Holla".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {} {}",ff,name);
    return none;
}


fn roulette3(name:String) -> String {
    let lao ="Sabaidee".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {} {}",lao,name);
    return none;
}


fn roulette4(name:String) -> String {
    let mar ="Namaskkaram".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {} {}",mar,name);
    return none;
}


fn roulette5(name:String) -> String {
    let mal ="Hai".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {}{}",mal,name);
    return none;
}


fn roulette6(name:String) -> String{
    let ne ="K cha".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {}{}",ne,name);
    return none;
}


fn roulette7(name:String) -> String{
    let ta ="Li-ho".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {}{}",ta,name);
    return none;
}


fn roulette8(name:String) -> String{
    let bb ="Xin chao".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {} {}",bb,name);
    return none;
}


fn roulette9(name:String) -> String{
    let fi ="Kamusta".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {} {}",fi,name);
    return none;
}


fn roulette10(name:String) -> String{
    let nem ="Hallo".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {} {}",nem,name);
    return none;
}


fn roulette11(name:String) -> String{
    let bas ="Kaixo".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {} {}",bas,name);
    return none;
}


fn roulette12(name:String) -> String{
    let kat ="Hola".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {} {}",kat,name);
    return none;
}


fn roulette13(name:String) -> String{
    let hor ="Bok".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {} {}",hor,name);
    return none;
}


fn roulette14(name:String) -> String{
    let bret ="Degemer mad".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {} {}",bret,name);
    return none;
}


fn roulette15(name:String) -> String{
    let dat ="Hej".to_string();
    let none = format!("Я научился приветствовать на 15 разных языках {} {}",dat,name);
    return none;
}