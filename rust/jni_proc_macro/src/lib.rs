
use class::{parse_class_with, func_transform_func};
use package::parse_package;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{ItemMod, LitStr};
    //todo: support option return type for null return
    //todo: add check for "C______S" and panic with "reserved keyword"
const PACKAGE_ARGUMENT_ERROR: &str = "jni_package accepts 1 argument that represents the package of the Java classes in this module. Example: #[jni_package(\"java.lang\")]";
const PACKAGE_ANNOTATED_TYPE_ERROR: &str = "jni_package can only be applied to modules";

#[proc_macro_attribute]
pub fn jni_package(args: TokenStream, package: TokenStream) -> TokenStream {
    if args.is_empty() {
        panic!("{}", PACKAGE_ARGUMENT_ERROR)
    }
    let args: Result<LitStr, _> = syn::parse(args);
    let Ok(package_name) = args else { panic!("{}", PACKAGE_ARGUMENT_ERROR)};
    let package: Result<ItemMod, _> = syn::parse(package);
    let Ok(mut package_mod) = package else { panic!("{}", PACKAGE_ANNOTATED_TYPE_ERROR)};

    parse_package(&mut package_mod, &package_name.value());
    TokenStream::from(package_mod.to_token_stream())
}

const CLASS_ARGUMENT_ERROR: &str = "jni_class accepts 1 argument that represents the name of the Java class this module is implementing. Example: #[jni_class(\"String\")]";
const CLASS_ANNOTATED_TYPE_ERROR: &str = "jni_class can only be applied to modules";

#[proc_macro_attribute]
pub fn jni_class(args: TokenStream, class: TokenStream) -> TokenStream {
    if args.is_empty() {
        panic!("{}", CLASS_ARGUMENT_ERROR)
    }
    let args: Result<LitStr, _> = syn::parse(args);
    let Ok(class_name) = args else { panic!("{}", CLASS_ARGUMENT_ERROR)};
    let class: Result<ItemMod, _> = syn::parse(class);
    let Ok(mut class_mod) = class else { panic!("{}", CLASS_ANNOTATED_TYPE_ERROR)};

    parse_class_with(&mut class_mod, |f| func_transform_func(f, &class_name.value()));
    TokenStream::from(class_mod.to_token_stream())
}

mod package {
    use syn::{ItemMod, Item, ItemFn};

    use crate::{class::parse_class_with, func::transform_func_name};



    pub fn parse_package(package_mod: &mut ItemMod, package_name: &str) {
        let Some((_, content)) = &mut package_mod.content else { return };
        content.iter_mut()
                .filter_map(|i| if let Item::Mod(module) = i { Some(module) } else { None })
                .for_each(|module| parse_class_with(module, |f| func_transform_func(f, package_name)));
    }

    fn func_transform_func(f: &mut ItemFn, package_name: &str) {
        let mut package_name = package_name.replace('.', "_");
        package_name.insert_str(0, "Java_");
        package_name.push_str("_C______S_");
        
        transform_func_name(f, |name| format!("{}{}",package_name, name));
    }
}

mod class {
    use syn::{ItemMod, Item, ItemFn};

    use crate::func::{transform_func_name, parse_func};

    pub fn parse_class_with<F>(class_mod: &mut ItemMod, f: F)
    where F : FnMut(&mut ItemFn) {
        let Some((_, content)) = &mut class_mod.content else { return };
        content.iter_mut()
                .filter_map(|i| if let Item::Fn(func) = i { Some(func) } else { None })
                .for_each(f);
    }

    pub fn func_transform_func(f: &mut ItemFn, class_name: &str) {
        transform_func_name(f, |name| name.replace("C______S", class_name)); 
        parse_func(f);
    }
}

mod func {


    use proc_macro2::{Ident, TokenStream};
    use syn::{
        parse_quote, punctuated::Punctuated, token::Comma, Block, FnArg, GenericArgument,
        ItemFn, Pat, PatType, PathArguments, ReturnType, Type,
    };

    type ResultType = (Type, Type);

    pub fn transform_func_name<F>(f: &mut ItemFn, func: F)
    where F : FnOnce(&str) -> String
    {
        let old_ident = f.sig.ident.clone();
        let new_ident = func(&old_ident.to_string());
        let new_ident: TokenStream = new_ident.parse().expect("failed to build func name");
        f.sig.ident = syn::parse2(new_ident).expect("failed to build func name");
    }

    pub fn parse_func(f: &mut ItemFn) {
        parse_abi(f);
        let result_type = parse_return_type(&mut f.sig.output);
        let runtime = parse_args(&mut f.sig.inputs);
        if let Some(result_type) = result_type {
            parse_body(&mut f.block, result_type, runtime);
        }
    }

    fn parse_abi(func: &mut ItemFn) {
        func.vis = parse_quote!(pub);
        func.sig.abi = Some(parse_quote!(extern "C"));
        func.attrs.push(parse_quote!(#[no_mangle]));
    }

    fn parse_return_type(output: &mut ReturnType) -> Option<ResultType> {
        let ReturnType::Type(_, ret_type) = output else { return None };
        let types = {
            let Type::Path(type_path) = &**ret_type else { return None };
            let last_segment = type_path.path.segments.iter().last();
            let Some(last_segment) = last_segment else { return None };
            if last_segment.ident != "Result" {
                return None;
            }
            let PathArguments::AngleBracketed(generic_args) = &last_segment.arguments else { return None };
            let mut generic_args_iter = generic_args.args.iter();
            if generic_args_iter.len() != 2 {
                return None;
            }
            let GenericArgument::Type(ok) = generic_args_iter.next()? else { return None };
            let GenericArgument::Type(err) = generic_args_iter.next()? else { return None };
            (ok.clone(), err.clone())
        };
        let _ = std::mem::replace(&mut **ret_type, types.0.clone());
        Some(types)
    }

    fn parse_args(args: &mut Punctuated<FnArg, Comma>) -> Ident {
        let runtime = args
            .iter()
            .filter_map(|arg| {
                if let FnArg::Typed(arg) = arg {
                    Some(arg)
                } else {
                    None
                }
            })
            .find_map(is_runtime_arg);
        match runtime {
            None => insert_runtime(args),
            Some(ident) => ident,
        }
    }

    fn insert_runtime(args: &mut Punctuated<FnArg, Comma>) -> Ident {
        let ident = parse_quote!(jre: JNIEnv);
        args.insert(0, ident);
        parse_quote!(jre)
    }

    fn is_runtime_arg(arg: &PatType) -> Option<Ident> {
        let Type::Path(type_path) = &*arg.ty else { return None };
        let last_segment = type_path.path.segments.iter().last();
        let Some(last_segment) = last_segment else { return None };
        if last_segment.ident != "JNIEnv" {
            return None;
        }
        let Pat::Ident(ident) = &*arg.pat else { return None };
        Some(ident.ident.clone())
    }

    fn parse_body(block: &mut Block, result_type: ResultType, runtime: Ident) {
        let old_body = block.clone();
        let (ok, err) = result_type;
        let new_body = parse_quote! {{
            let body = || #old_body;
            let result: Result<#ok, #err> = body();
            match result {
                Err(err) => {
                    match #runtime.throw(err) {
                        Ok(_) => {},
                        Err(err) => println!("ExceptionError:{:?}", err)
                    }
                    unsafe { std::mem::zeroed() } // Will be discarded by jvm since Exception was thrown.
                },
                Ok(result) => result
            }
        }};
        *block = new_body;
    }
}
