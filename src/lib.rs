//! Easy way to duplicate a new function with `self: Box<Self>` signature.
//!
//! Sometimes you need both functions `fn consume(self)` and `fn consume_boxed(self: Box<Self>)`. This macro generates the second one for you.
//! 
//! # Examples
//! ```
//!    use box_self::box_self; 
//!
//!    trait Animal {
//!        fn consume(self);
//!        fn consume_boxed(self: Box<Self>);
//!    } 
//!
//!    struct Dog{}
//!    impl Animal for Dog{
//!        #[box_self(_boxed)]
//!        fn consume(self) {
//!            println!("Bark");
//!        }
//!    } 
//!
//!    struct Cat{}
//!    impl Animal for Cat{
//!        #[box_self(_boxed)]
//!        fn consume(self) {
//!            println!("Jump");
//!        }
//!    } 
//!
//!    fn main(){
//!        let mut animals:Vec<Box<dyn Animal>>=Vec::new();
//!        animals.push(Box::new(Dog{}));
//!        animals.push(Box::new(Cat{}));
//!        for anim in animals{
//!            anim.consume_boxed()
//!        }
//!    }
//! ```
//! 
//! <br><br>
//! ### Motivation:
//! - [`How to call a method that consumes self on a boxed trait object?`]
//!
//! [`How to call a method that consumes self on a boxed trait object?`]: https://stackoverflow.com/questions/46620790/how-to-call-a-method-that-consumes-self-on-a-boxed-trait-object
//! <br>
//! 
//! ### Another solution on nightly Rust `unsized_fn_params`:
//! - [`How to pass a boxed trait object by value in Rust?`]
//! 
//! [`How to pass a boxed trait object by value in Rust?`]: https://stackoverflow.com/questions/65261399/how-to-pass-a-boxed-trait-object-by-value-in-rust
//! <br>

use proc_macro::{TokenStream, TokenTree,  Ident};
use quote::{quote,ToTokens};
use syn::{parse_macro_input, ItemFn};

#[doc(hidden)]
#[proc_macro]
pub fn replace_params(input: TokenStream) -> TokenStream {
    let mut old_params=input.into_iter();
    let mut new_params=Vec::new();
    while let Some(tt) =old_params.next(){
        match tt {
            TokenTree::Ident(i) if i.to_string()=="self" =>{   
                new_params.push(TokenTree::Ident(i));
                let extra_boxed:TokenStream=quote!{:Box<Self>}.into();      //add ':Box<Self>'
                for extra_boxed_param in extra_boxed.into_iter(){
                    new_params.push(extra_boxed_param);
                }
            },
            // All other tokens are just forwarded
            other =>{
                new_params.push(other)
            }
        }
    }
    new_params.into_iter().collect()
}

/// Duplicate `consume self` function with boxed signature and postfix
#[proc_macro_attribute]
pub fn box_self(attr_postfix: TokenStream, item: TokenStream) -> TokenStream {
    // get the function this attribute is attached to
    let orig_func = parse_macro_input!(item as ItemFn);
    let orig_func_stream:TokenStream=orig_func.to_token_stream().into();

    let mut next_group_is_params=false;
    let mut next_iden_is_fn_name=false;
    let it=orig_func_stream.into_iter();
    let boxed_self_func:TokenStream=it.map(|tt| {
        match tt {
            TokenTree::Ident(ref i) if i.to_string()=="fn"=>{
                next_iden_is_fn_name=true;
                TokenTree::Ident(i.clone())
            },
            TokenTree::Ident(ref i) if next_iden_is_fn_name => {
                next_iden_is_fn_name=false;
                next_group_is_params=true;
                TokenTree::Ident(Ident::new((i.to_string()+attr_postfix.to_string().as_str()).as_str(), i.span()))
            },
            TokenTree::Group(params) if next_group_is_params =>{
                next_group_is_params=false;
                let new_stream=replace_params(params.stream());
                let g2=proc_macro::Group::new(params.delimiter(),new_stream);
                TokenTree::Group(g2)
            },
            // All other tokens are just forwarded
            other => other,
        }
    }).collect();

    TokenStream::from_iter( [orig_func.to_token_stream().into() ,boxed_self_func])
}   
