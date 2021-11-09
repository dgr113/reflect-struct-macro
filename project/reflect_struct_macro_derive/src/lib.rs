extern crate proc_macro;
#[macro_use] extern crate syn;

use quote::quote;
use proc_macro::{ TokenStream };




fn impl_reflect_struct_macro( input: TokenStream ) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse( input.clone() ).unwrap();
    let struct_name = &ast.ident;
    let struct_item = parse_macro_input!( input as syn::ItemStruct );

    let fields_names = struct_item.fields.iter()
        .map( |f| f.ident.clone().unwrap().to_string() )
        .collect::<Vec<String>>();

    let fields_count = &fields_names.len();

    let gen = quote! {
        impl<'a> ReflectStructMacro<'a, #fields_count> for #struct_name {
            fn list_fields_names() -> [&'a str; #fields_count] {
                [ #( #fields_names ), * ]
            }
            fn fields_count() -> usize {
                #fields_count
            }
        }
    };

    gen.into()
}



#[proc_macro_derive(ReflectStructMacro)]
pub fn reflect_struct_macro_derive( input: TokenStream ) -> TokenStream {
    impl_reflect_struct_macro( input )
}
