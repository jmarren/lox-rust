use proc_macro::TokenStream;
use proc_macro2::{Punct, TokenStream as TokenStream2, TokenTree};
use quote::{ ToTokens, TokenStreamExt, quote };


#[proc_macro]
pub fn to_struct(_input: TokenStream) -> TokenStream {
    // let input = TokenStream2::from(input);
    
    quote! {
        pub struct HandleBar {}
    }.into()
}

  // private static void defineType(
  //     PrintWriter writer, String baseName,
  //     String className, String fieldList) {
  //   writer.println("  static class " + className + " extends " +
  //       baseName + " {");
  //
  //   // Constructor.
  //   writer.println("    " + className + "(" + fieldList + ") {");
  //
  //   // Store parameters in fields.
  //   String[] fields = fieldList.split(", ");
  //   for (String field : fields) {
  //     String name = field.split(" ")[1];
  //     writer.println("      this." + name + " = " + name + ";");
  //   }
  //
  //   writer.println("    }");
  //
  //   // Fields.
  //   writer.println();
  //   for (String field : fields) {
  //     writer.println("    final " + field + ";");
  //   }
  //
  //   writer.println("  }");
  // }
  //

#[derive(Debug)]
struct Property<'a> {
    name: &'a TokenTree,
    typ: &'a TokenTree,
}

impl <'a>ToTokens for Property<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = self.name;
        let typ = self.typ; 

        // let comma = quote!{,};


        quote! { 
            pub #name: #typ ,
        }.to_tokens(tokens);

    }
}



#[proc_macro]
pub fn expression(original_input: TokenStream) -> TokenStream {
    // let input = TokenStream2::from(input);
    
    let input: TokenStream2 = original_input.into();

    let inputs: Vec<TokenTree> = input.into_iter().collect();

        // .into_iter()
        // .collect();

        // .filter(| token | {
        //     match token {
        //         TokenTree::Ident(ident) => {
        //             true       
        //         },
        //         _ => false,
        //     }
        // });
        //

    
    let struct_name = inputs.first().unwrap();

    let rest = &inputs[1..];

    let mut i = 1;

    let mut props = vec![];

    while i < rest.len() {
        props.push(Property{
            name: &rest[i - 1],
            typ: &rest[i],
        });
        i+=2;
    }

    println!("props = {:?}", props);

    // let second = inputs.iter().nth(1).unwrap();

    
    // match second {
    //     TokenTree::Group(g) => {
    //         println!("got group = {:?}", g);
    //     },
    //     _=>(),
    // }

    // println!("span = {:?}", prop_name.span());
    //
    // println!("prop_name = {:?}", prop_name);
    //
    let out = quote! {
        pub struct #struct_name {
            #(#props)*
        }
    }.into();

    // String::from(out);
        
    println!("out = {:?}", out);
    
    out

}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
//
//
