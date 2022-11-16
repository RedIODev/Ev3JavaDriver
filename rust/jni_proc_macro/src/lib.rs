use proc_macro::TokenStream;
use quote::{quote,ToTokens};
use syn::{ItemFn, Item, ReturnType,Type, PathArguments, GenericArgument, parse_quote};
                                                                                                                        //todo: remove unwrap and generate usable error

#[proc_macro_attribute]
pub fn jni_func(args: TokenStream, func:TokenStream) -> TokenStream {
    func
}


#[proc_macro_attribute]
pub fn jni_mod(args: TokenStream, module: TokenStream) -> TokenStream {    //todo: impl name mangling based on args
    let mut module :syn::ItemMod = syn::parse(module).unwrap();
    let (brace, funcs) = module.content.unwrap();
    let (funcs, mut rest):(Vec<Item>,Vec<Item>) = funcs.into_iter().partition(|x| matches!(x, Item::Fn(_)));
    let funcs = funcs
            .into_iter()
            .filter_map(|x| if let Item::Fn(f) = x { Some(f)} else { None })
            .map(parse_fn);
    rest.extend(funcs);
    module.content = Some((brace,rest));
    TokenStream::from(module.to_token_stream())
}


fn parse_fn(mut f: ItemFn) -> Item {    //todo: func(jre) -> (), func() -> Result<..>, func() -> ()
    f.vis = parse_quote!(pub);
    f.sig.abi = Some(parse_quote!(extern "system"));
    f.attrs.push(parse_quote!(#[no_mangle]));
    let (return_type, err_type) = get_result_ok_type(f.sig.output.clone()).unwrap();
    let ReturnType::Type(_, t) = &mut f.sig.output else { panic!("Illegal ReturnType'")};
    *t = Box::new(return_type.clone());
    let body = f.block;
    let new_body = quote!{{
        let f = || #body;
        let result: Result<#return_type,#err_type> = f();
        match result {
            Err(err) => {
                jre.throw(err).unwrap();
                unsafe {std::mem::zeroed()}
            },
            Ok(r) => r,
        }
    }};
    f.block = Box::new(syn::parse2(new_body).unwrap());
    Item::Fn(f)
}

fn get_result_ok_type(out: ReturnType) -> Option<(Type,Type)> {
    let ReturnType::Type(_, ret_type) = out else { return None; };
    let Type::Path(ret_type_path) = * ret_type else { return None; };
    let ret_type_path_segments = ret_type_path.path.segments.into_iter();
    let result_type = ret_type_path_segments.last().unwrap();
    if result_type.ident.to_string()!="Result" {return None; }
    let result_type_args = result_type.arguments;
    let PathArguments::AngleBracketed(result_generic_args) = result_type_args else { return None; };
    let mut result_generic_args = result_generic_args.args.into_iter();
    let GenericArgument::Type(ok_type) = result_generic_args.next().unwrap() else { return None;};
    let GenericArgument::Type(err_type) = result_generic_args.next().unwrap() else { return None; };
    Some((ok_type,err_type))
}