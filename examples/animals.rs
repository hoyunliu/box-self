use box_self::box_self; 

trait Animal {
    fn consume(self);
    fn consume_boxed(self:Box<Self>);
} 

struct Dog{}
impl Animal for Dog{
    #[box_self(_boxed)]
    fn consume(self) {
        println!("Bark");
    }
} 

struct Cat{}
impl Animal for Cat{
    #[box_self(_boxed)]
    fn consume(self) {
        println!("Jump");
    }
} 


fn main(){
    let animals:Vec<Box<dyn Animal>>=vec![Box::new(Dog{}), Box::new(Cat{})];
    
    for anim in animals{
        anim.consume_boxed();
    }
}

