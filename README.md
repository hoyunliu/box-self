# box-self

Easy way to duplicate a new function with `self: Box<Self>` signature.

Sometimes you need both functions `fn consume(self)` and `fn consume_boxed(self: Box<Self>)`. This macro generates the second one for you.

## Examples
```rust
   use box_self::box_self;

   trait Animal {
       fn consume(self);
       fn consume_boxed(self: Box<Self>);
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
       let animals:Vec<Box<dyn Animal>>=
            vec![Box::new(Dog{}), Box::new(Cat{})];

       for anim in animals{
           anim.consume_boxed();
       }
   }
```

<br><br>
#### Motivation:
- [`How to call a method that consumes self on a boxed trait object?`]

[`How to call a method that consumes self on a boxed trait object?`]: https://stackoverflow.com/questions/46620790/how-to-call-a-method-that-consumes-self-on-a-boxed-trait-object
<br>


#### License
Licensed under either of [LICENSE-APACHE](LICENSE-APACHE) or [LICENSE-MIT](LICENSE-MIT)  at your option.

<br>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

License: MIT OR Apache-2.0
