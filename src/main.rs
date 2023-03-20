
mod gauss;
mod input;

use crate::gauss::linear;
use std::vec;




fn main() {

    match linear::LinearSystem::new(
        vec![
            vec![-2.0, 1.0, 0.0, 1.0],
            vec![0.0, 2.0, 1.0, 1.0],
            vec![12.0,7.0,2.0, 1.0],
            vec![52.0, 16.0, 3.0, 1.0]
        ], vec![2.0, 4.0, 10.0, 26.0]) {
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