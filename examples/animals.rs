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
    let mut animals:Vec<Box<dyn Animal>>=Vec::new();
    animals.push(Box::new(Dog{}));
    animals.push(Box::new(Cat{}));
    for anim in animals{
        anim.consume_boxed();
    }
}
