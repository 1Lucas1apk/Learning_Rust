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

	// tipos de dados;

	// tipos de inteiros são relacionados com a capacidade de bits de armazenamento.
	// i8 (8 bit) i16 (16 bit) i32 (32 bit) i64 (64 bit) i128 (128 bit) isize (iside bit);

	// O u significa unsigned, ou seja, não negativo, enquanto i significa signed, ou seja, negativo, o numero que o segue é o número de bit, por exemplo, um numero do tipo u8 pode estar entre 0 e 255, já um numero do tipo i8 pode estar entre -128 e 127;

	// O f significa float, ou seja, um numero quebrado, por exemplo, o número 1.5 é um float;

	// O bool significa booleano, ou seja, verdadeiro ou falso uma variavel com essa atribuição pode ser; (true, false)

  let inteiro: i32 = -1;
	let nao_inteiro: u32 = 1;
  let flotoante: f32 = 1.5;

	let booleano: bool = true;

	println!("Número inteiro: {}, número não inteiro {}, número flotoante: {} e booleano: {}", inteiro, nao_inteiro, flotoante, booleano);
}