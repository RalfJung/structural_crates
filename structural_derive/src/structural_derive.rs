use crate::{
    tokenizers::NamedModuleAndTokens,
    structural_alias_impl::{IdentOrIndexRef,StructuralAliasFieldRef},
};

use as_derive_utils::{
    datastructure::{DataStructure,DataVariant,Field},
    gen_params_in::{GenParamsIn,InWhat},
    return_syn_err,
    ToTokenFnMut,
};

use core_extensions::SelfOps;

use proc_macro2::{
    TokenStream as TokenStream2,
    Span,
};

use quote::{quote,ToTokens};

use syn::{
    punctuated::Punctuated,
    DeriveInput,
    Ident,
    Token,
};


mod attribute_parsing;


pub fn derive(data: DeriveInput) -> Result<TokenStream2,syn::Error> {
    let ds = &DataStructure::new(&data);

    match ds.data_variant {
        DataVariant::Enum=>
            return_syn_err!(Span::call_site(),"Cannot derive Structural on an enum"),
        DataVariant::Union=>
            return_syn_err!(Span::call_site(),"Cannot derive Structural on an union"),
        DataVariant::Struct=>{}            
    }

    let config=&attribute_parsing::parse_attrs_for_structural(ds)?;
    
    let struct_=&ds.variants[0];

    let fields=config.fields.values()
        .zip( struct_.fields.iter() )
        .filter(|(f_cond,_)| f_cond.is_pub )
        .map(|(_,f)| f )
        .collect::<Vec<&Field<'_>>>();
    
    let names_module_definition=NamedModuleAndTokens::new(
        ds.name,
        fields.iter().cloned().map(|f| &f.ident )
    );
    
    let names_module=&names_module_definition.names_module;
    let alias_names=&names_module_definition.alias_names;
    
    let field_names=config.fields.values()
        .zip( fields.iter().cloned() )
        .map(|(field_conf,field)|{
            ToTokenFnMut::new(move|ts|{
                match &field_conf.renamed {
                    Some(x) => x.to_tokens(ts),
                    None => field.ident().to_tokens(ts),
                }
            })
        });

    let field_tys=fields.iter().cloned().map(|f| &f.ty );

    let impl_generics=GenParamsIn::with_after_types(
        ds.generics,
        InWhat::ImplHeader,
        quote!(),
    );

    let (_, ty_generics, where_clause) = ds.generics.split_for_impl();

    let tyname=ds.name;

    let empty_preds=Punctuated::new();
    let where_preds=where_clause.as_ref().map_or(&empty_preds,|x|&x.predicates).into_iter();

    let getter_trait=config.fields.values().map(|f| f.access );

    // dbg!(field_names.clone().for_each(|x|{ dbg!(x.to_token_stream().to_string()); }));
    // dbg!(&field_tys);
    // dbg!(alias_names.iter().for_each(|x|{ dbg!(x); }));

    let docs=format!(
        "A trait aliasing the accessor impls for \
         [{struct_}](./struct.{struct_}.html) fields\n\
         \n\
         This trait also has all the constraints(where clause and generic parametr bounds)
         of [the same struct](./struct.{struct_}.html).\n\n\
         ### Accessor traits\n\
         These are the accessor traits this aliases:\n\
        ",
        struct_=tyname,
    );

    let structural_alias_trait=crate::structural_alias_impl::for_delegation(
        std::iter::empty::<Ident>(),
        docs,
        ds.vis,
        &<Token!(trait)>::default(),
        &Ident::new(&format!("{}_SI",tyname),Span::call_site()),
        ds.generics,
        &Punctuated::new(),
        &names_module_definition,
        fields.iter()
            .zip(config.fields.values())
            .map(|( field, field_config )|{
                StructuralAliasFieldRef{
                    access:field_config.access,
                    ident:field.ident().piped(IdentOrIndexRef::Ident),
                    ty:field.ty,
                }
            }),
    )?;

    quote!(
        #structural_alias_trait

        impl_getters_for_derive!{
            impl[#impl_generics] #tyname #ty_generics
            where[ #(#where_preds)* ] 
            {
                #((
                    #getter_trait< 
                        #field_names : #field_tys , 
                        #names_module::#alias_names> 
                ))*
            }
        }
    )
    .observe(|tokens|{
        if config.debug_print{
            panic!("\n\n\n{}\n\n\n",tokens);
        }
    })
    .piped(Ok)
}
