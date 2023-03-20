

pub mod linear {
    use std::{fmt::{self, Display}};

    pub type Vector = Vec<f64>;
    pub type Matrix = (Vector, usize);

    pub struct LinearSystem(Matrix, Vector);

    impl Display for LinearSystem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut buf = String::new();
            for i in 0..self.0.1 {
                for j in 0..self.0.1 {
                    buf.push_str(&format!("{:0.2} ", self.0.0[i*self.0.1 + j]));
                }
                buf.push_str(&format!("= {:0.2}\n", self.1[i]));
            }
            write!(f, "{}", buf)
        }
    }


    impl LinearSystem {

        pub fn new(a: Vec<Vec<f64>>, b: Vector) -> Result<Self, &'static str> {
            let n = a.len();
            let mut buffer = vec![0.0; n*n];
            for i in 0..n {
                if a[i].len() != n {
                    return Err("invalid system");
                }
                for j in 0..n {
                    buffer[i*n + j] = a[i][j];
                }
            }
            Ok(Self((buffer, n), b))
        }


        pub fn solve(&self) -> Result<LinearSystem, &'static str> {

            let (arr,n) = &self.0;
            let n = *n;
            let b = &self.1;
            let system = Self::new(
                vec![vec![0.0; n]; n], vec![0.0; n]
            ).unwrap();
            
            let ((mut res_arr, _), mut res_vec) = (system.0, system.1);

            for i in 0..n {
                for j in 0..n {
                    res_arr[i*n+j] = arr[i*n+j];
                }
                res_vec[i] = b[i];
            }
            
            for i in 0..n {
                let coef = res_arr[i*n+i];
                if coef == 0.0 {
                    if i != n-1 {
                        return Err("division by zero");
                    }
                    
                }
                for j in i..n {
                    res_arr[i*n+j] /= coef;
                }
                res_vec[i] /= coef;


                for k in i+1..n {
                    let mut next_row = vec![0.0;n];
                    for j in i..n {
                        
                        if res_arr[k*n+i] == 0.0 {
                            for t in k..n {
                                if res_arr[t*n+i] != 0.0 {
                                    for x in i..n {
                                        let temp  = res_arr[t*n + x];
                                        res_arr[t*n+x] = res_arr[k*n+x];
                                        res_arr[k*n+x] = temp;
                                        
                                    }
                                    break;
                                }
                            }
                        }
                        next_row[j] = -res_arr[k*n+j]/res_arr[k*n+i] + res_arr[i*n+j];
                        
                    }
                    res_vec[k] = -res_vec[k]/res_arr[k*n+i] + res_vec[i];
                    for x in 0..n {
                        res_arr[k*n+x] = next_row[x];
                    }
                }

            } 
            for k in (0..n).rev() {
                for i in (0..k).rev() {
                    println!("({k} : {i})");
                    let coef = res_arr[i*n+k];
                    if coef == 0.0 {
                        continue;
                    }
                    res_arr[i*n+k] -= res_arr[k*n+k]*coef;
                    res_vec[i] -= res_vec[k]*coef;
                }
            }
            Ok(Self((res_arr, n), res_vec))
        }

    }
}

