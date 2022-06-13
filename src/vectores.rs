pub mod vector{
    pub fn vectores(vect: std::vec::Vec<i32>) {
        let mut v:Vec<i32> = Vec::new();
        
        for num in vect {
            v.push(num);
        }

        for i in &v {
            println!("{}",i);
        }

        println!("El segundo elemento del vector es: {}", &v[1]);

        match v.get(2) {
            Some(v) => println!("El tercer elemento del vector es: {}", v),
            None => println!("No hay tercer elemento en el vector"),
        }
}
}

