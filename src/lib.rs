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
//!        let animals:Vec<Box<dyn Animal>>=
//!             vec![Box::new(Dog{}), Box::new(Cat{})];
//!        
//!        for anim in animals{
//!            anim.consume_boxed();
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
//! 
//! ### License
//! Licensed under either of [LICENSE-APACHE](LICENSE-APACHE) or [LICENSE-MIT](LICENSE-MIT)  at your option.
//! 
//! <br>
//! 
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
//! be dual licensed as above, without any additional terms or conditions.



use proc_macro::{TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote,ToTokens};
use syn::{parse_macro_input, ItemFn};




#[proc_macro_attribute]
pub fn box_self(attr_postfix: TokenStream, item: TokenStream) -> TokenStream {
    let orig_func = parse_macro_input!(item as ItemFn);

    let func_vis = &orig_func.vis; 
    let func_sig = orig_func.sig.clone();
    let func_generics = &func_sig.generics;
    let func_output = &func_sig.output;

    // original function signature
    let orgi_fn_name=&func_sig.ident;
    let s_fn_name=orgi_fn_name.to_string()+attr_postfix.to_string().as_str();
    let func_params = func_sig.inputs.clone();

    // convert to new function signature
    let new_func_name: proc_macro2::Ident = proc_macro2::Ident::new(s_fn_name.as_str(),proc_macro2::Span::call_site()); // function name
    let delcaring_params=replace_params_declaration(func_params.to_token_stream().into());
    let calling_params=extract_params_without_type(func_params.into_token_stream());
    
    // generate the new function body,  using  `  (*self).consume(calling_params)  `
    let new_func:TokenStream = quote!{
        #[inline] #func_vis fn #new_func_name #func_generics(#delcaring_params) #func_output {
            (*self).#orgi_fn_name(#calling_params)
        }
    }.into();
 
    TokenStream::from_iter( [orig_func.to_token_stream().into() ,new_func])
}


//replace 'self' with 'self:Box<Self>'  in the parameters declaration
fn replace_params_declaration(input: TokenStream2) -> TokenStream2 {
    use proc_macro2::TokenTree;
    let mut old_params=input.into_iter();
    let mut new_params=Vec::new();
    while let Some(tt) =old_params.next(){
        match tt {
            proc_macro2::TokenTree::Ident(i) if i.to_string()=="self" =>{   
                new_params.push(TokenTree::Ident(i));
                let extra_boxed:TokenStream2=quote!{:Box<Self>}.into();      //add ':Box<Self>'
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

// extract parameters for calling the original function
fn extract_params_without_type(params:TokenStream2) -> TokenStream2 {
    let mut params_without_type=Vec::new();
    let mut it=params.into_iter();
    let mut last_ident:Option::<proc_macro2::Ident>=None;

    // only take idents before ':' 
    let mut ident_not_found=true;
    while let Some(tt) =it.next(){
        //println!("tt={tt:?}");
        match tt {
            proc_macro2::TokenTree::Ident(i) =>{
                last_ident=Some(i.clone());
                
            },
            proc_macro2::TokenTree::Punct(p) if p.as_char()==':' && ident_not_found  =>{
                if let Some(i)=&last_ident{
                    if i.to_string()!="self"{ // ignore 'self'
                       //println!("pushing ident={}",i.to_string());
                        ident_not_found=false;
                        params_without_type.push(proc_macro2::TokenTree::Ident(last_ident.take().unwrap()));
                        let p=proc_macro2::Punct::new(',',proc_macro2::Spacing::Alone);
                        params_without_type.push(proc_macro2::TokenTree::Punct(p));
                    }
                }
            },
            proc_macro2::TokenTree::Punct(p) if p.as_char()==',' =>{
                ident_not_found=true; // reset for next ident
            }
            _=>{}
        }
    }
    params_without_type.into_iter().collect()
}
