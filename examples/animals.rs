use box_self::box_self; 

trait Animal {
    fn consume(self);
    fn consume_boxed(self:Box<Self>);
    fn extra(&self);
} 

struct Dog{}
impl Animal for Dog{
    #[box_self(_boxed)]
    fn consume(mut self) {
        println!("Bark");
    }
    fn extra(&self){

    }
} 

struct Cat{}
impl Animal for Cat{
    #[box_self(_boxed)]
    fn consume(mut self) {
        println!("Jump");
    }
    fn extra(&self){
        
    }
} 

impl Animal for Box<dyn Animal>{
    fn consume(self) {
        Animal::consume_boxed(self);
    }

    fn consume_boxed(self:Box<Self>) {
        Animal::consume_boxed(*self);
    }

    fn extra(&self){
       (**self).extra(); 
    }
} 

fn main(){
    let mut animals:Vec<Box<dyn Animal>>=Vec::new();
    animals.push(Box::new(Dog{}));
    animals.push(Box::new(Cat{}));
    for anim in animals{
        anim.consume_boxed();
    }
}