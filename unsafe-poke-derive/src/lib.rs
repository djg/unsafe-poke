use quote::quote;
use synstructure::{decl_derive, Structure};

fn unsafe_poke_derive(s: Structure) -> proc_macro2::TokenStream {
    // a single variant with no prefix is 'struct' so we don't need a discriminant
    let is_struct = match &s.variants()[..] {
        [v] if v.prefix.is_none() => true,
        _ => false,
    };

    let body = s
        .variants()
        .iter()
        .enumerate()
        .fold(quote!(), |acc, (i, variant)| {
            let pat = variant.pat();
            let pokes = variant.bindings().iter().fold(
                if is_struct {
                    quote!()
                } else {
                    quote!(let up = (#i as u32).poke(up); )
                },
                |acc, binding| {
                    let binding = &binding.binding;
                    quote!(#acc let up = #binding.poke(up);)
                },
            );
            quote!(#acc #pat => { #pokes up })
        });

    s.bound_impl(
        quote!(unsafe_poke::UnsafePoke),
        quote!(
            fn poke<UP>(&self, up: UP) -> UP
            where
                UP: unsafe_poke::UnsafePokable
            {
                match *self {
                    #body
                }
            }
        ),
    )
}

decl_derive!([UnsafePoke] => unsafe_poke_derive);

#[cfg(test)]
mod tests {
    use syn;
    use synstructure;

    #[test]
    fn it_works() {
        let source = syn::parse_str(
            //"enum Foo<T> { M(i32), O(Bar, T, Arc<T>) }",
            // "enum Foo<T> { M(Option<i32>) }",
            "struct Bar { a: u32, b: u32, c: u32, d: Option<u32>, }",
        )
        .unwrap();
        let source = synstructure::Structure::new(&source);

        let expanded = crate::unsafe_poke_derive(source).to_string();
        println!("{}", expanded);
    }
}
