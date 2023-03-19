
mod gauss;

use crate::gauss::linear;
use std::vec;




fn main() {

    match linear::LinearSystem::new(
        vec![
            vec![-2.0, 1.0, 5.0],
            vec![-4.0, 5.0, 7.0],
            vec![1.0,1.0,1.0],
        ], vec![1.0, 4.0, 2.5]) {
            Ok(system) => {
                println!("{}", system);
                match system.solve() {
                    Ok(solution) => {
                        println!("{:=<16}","");
                        println!("{}", solution);
                    },
                    Err(reason) => eprintln!("{reason}")
                }
            
                                        
                
            },
            Err(reason) => eprintln!("{reason}")
        }
}