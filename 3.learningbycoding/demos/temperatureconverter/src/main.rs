//典型温度
//°C	°F	描述
// 180	356	烤箱中等温度
// 100	212	水沸
// 40	104	热水浴
// 37	98.6	体温
// 30	86	沙滩天气
// 21	70	室温
// 10	50	凉快天气
// 0	32	水结冰
// −18	0	非常寒冷天气
// −40	−40	极端寒冷天气
// （度数一样！）

use std::io;
fn main() {

    loop { 
        println!("please input Y or N, if you want celsuis_to_fahrenheit,input Y; if you want fahrenheit_to_celsius,input N; input E to exit");

        let mut switch = String::new();
        io::stdin().read_line(&mut switch).expect("Failed to read line");

        let switch: char = match switch.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("you need to input Y or N (input E to exit)!");
                continue;
            }
        };

        if switch == 'Y' {//摄氏温度转换成华氏温度
            loop {
                println!("Please enter a temperature in Celsuis(input E to exit):");
                let mut input = String::new();
        
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let input: f32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        if input.trim() == "E" {
                            break;
                        }

                        println!("You need to input a number!");
                        continue;
                    }
                };
                
                let fahrenheit = celsuis_to_fahrenheit(input);
                println!("Celsuis temperature {} is {} Fehenheit", input, fahrenheit);
            }
           
        }else if switch == 'N' {// 华氏温度转换成摄氏温度
            loop {
                println!("Please enter a temperature in Fehenheit(input E to exit):");
                let mut input = String::new();
    
                io::stdin().read_line(&mut input).expect("Failed to read line");

                let input: f32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        if input.trim() == "E" {
                            break;
                        }
                        println!("You need to input a number!");
                        continue;
                    }
                };

                let celsius = fahrenheit_to_celsius(input);
                println!("Fehenheit temperature {} is {} Celsius", input, celsius);
            }  
        }else if switch == 'E' {
            break;
        }

        continue; 
}
    
}

fn celsuis_to_fahrenheit(celsuis: f32) -> f32 {
     &celsuis * 9.0 / 5.0 + 32.0
}


fn fahrenheit_to_celsius(fahrenheit: f32) -> f32{
    (&fahrenheit - 32.0) * 5.0 / 9.0
}