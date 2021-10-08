

struct Pet<'a>{
	name: &'a str,
	cry : &'a str
}

impl<'a> Pet<'a>{

	fn cry(&self){
		println!("{} cries {}", self.name, self.cry);
	}

}

fn main(){

	let pikachu = Pet{name: "Pikachu", cry: "*Pika Pika*"};
	pikachu.cry();

}
