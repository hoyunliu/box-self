use async_trait::async_trait;
use box_self::box_self;

mod private{
    pub struct MyBool(pub bool);
}



#[async_trait]
trait Animal {
    async fn consume(self,age:u32, b:private::MyBool)->u32;
    async fn consume_boxed(self: Box<Self>, age:u32,b:private::MyBool)->u32;
}

struct Dog{}

#[async_trait]
impl Animal for Dog{
    #[box_self(_boxed)]
    async fn consume(self,mut age: u32, b: private::MyBool)-> u32 {
        println!("Bark");
        if b.0 {
            age=age+1;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        return age;
    }
}



struct Cat{}

#[async_trait]
impl Animal for Cat{
    #[box_self(_boxed)]
    async fn consume(self,mut age: u32,b: private::MyBool) ->u32{
        println!("Jump");
        if b.0 {
            age=age+1;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        return age;
    }
    
}

#[tokio::main]
async fn main(){
    let animals:Vec<Box<dyn Animal>>=
         vec![Box::new(Dog{}), Box::new(Cat{})];

    for (i,anim) in animals.into_iter().enumerate(){
        println!("age={}",anim.consume_boxed(i as u32 ,private::MyBool(true)).await);
    }
}