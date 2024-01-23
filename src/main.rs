fn main() {
    println!("Hello World!");

	  // variavel 

	  let name: &str = "Lucas";
	  println!("Meu nome é {}", name);
	
	 // variável é imutável
   // name = "Marcos"; (não é possível!)

 	// let age = 17;
	// age += 1; (erro porque é umutavel);

	// tranformar a variavel em mutável

  let mut age: i32 = 17;
	age += 1;
	println!("Irei fazer {} anos", age);

	// assim trabforma uma variável imutável em mutável;
}