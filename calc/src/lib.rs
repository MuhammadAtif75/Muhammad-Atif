pub mod calculator_functions{
    pub mod basic_functions{
        pub fn addition(a:u32,b:u32)->u32{
            a+b
        }
        pub fn subtraction(a:u32,b:u32)->u32{
            a-b
        }
        pub fn mutliply(a:u32,b:u32)->u32{
            a*b
        }
        pub fn divide(a:u32,b:u32)->u32{
            a/b
        }
    }
    pub mod power_functions{
        pub fn square(a:u32)->u32{
            a*a
        }
        pub fn cube(a:u32)->u32{
            a*a*a
        }
        pub fn power(number:i32,power:i32)->i32{
            let mut count = 1; 
            for _i in 1..(power+1){
                let result = number * count;
                count = result;        
            }
            count
        }
    } 
} 