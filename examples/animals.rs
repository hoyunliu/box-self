use box_self::box_self;
trait Animal {
    fn consume(self,age:u32, b:bool)->u32;
    fn consume_boxed(self: Box<Self>, age:u32,b:bool)->u32;
}

struct Dog{}
impl Animal for Dog{
    #[box_self(_boxed)]
    fn consume(self,mut age: u32, b:bool)-> u32 {
        println!("Bark");
        if b {
            age=age+1;
        }
        return age;
    }
}

struct Cat{}
impl Animal for Cat{
    #[box_self(_boxed)]
    fn consume(self,mut age: u32,b:bool) ->u32{
        println!("Jump");
        if b {
            age=age+1;
        }
        return age;
    }
    
}

fn main(){
    let animals:Vec<Box<dyn Animal>>=
         vec![Box::new(Dog{}), Box::new(Cat{})];

    for (i,anim) in animals.into_iter().enumerate(){
        println!("age={}",anim.consume_boxed(i as u32 ,true));
    }
}