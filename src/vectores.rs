pub mod vector{
    pub fn vectores(vect: std::vec::Vec<i32>) {
        let mut v:Vec<i32> = Vec::new();
        
        for num in vect {
            v.push(num);
        }

        for i in v {
            println!("{}",i);
        }
}
}

