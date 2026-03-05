use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, GenericArgument, PathArguments, Type, TypePath, parse_macro_input};

fn extract_option_inner_ty(ty: &syn::Type) -> Option<&syn::Type> {
    // Match `Option<...>` or `std::option::Option<...>`
    let Type::Path(TypePath { qself: None, path }) = ty else {
        return None;
    };

    let seg = path.segments.last()?; // last segment: `Option`
    if seg.ident != "Option" {
        return None;
    }

    // Look at the generic arguments on `Option`
    let PathArguments::AngleBracketed(ref abga) = seg.arguments else {
        return None;
    };

    // Expect a single type argument: `Option<T>`
    let GenericArgument::Type(inner_ty) = abga.args.first()? else {
        return None;
    };

    Some(inner_ty)
}

#[proc_macro_derive(DicomCompositeObject)]
pub fn dicom_composite_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(data) => {
            let fields = &data.fields;
            fields.iter().collect::<Vec<_>>()
        }
        &syn::Data::Enum(_) | &syn::Data::Union(_) => vec![],
    };

    let mut read_values = vec![];
    for field in fields {
        let ident = &field.ident;
        let ty = &field.ty;
        if let Some(ident) = ident {
            if let Some(inner) = extract_option_inner_ty(ty) {
                read_values.push(quote! {
                    #ident: <#inner as rad_tools_dcm_value::ReadDicomValue<dicom_object::InMemDicomObject>>::read_value_opt(backend)?
                });
            } else {
                read_values.push(quote! {
                    #ident: <#ty as rad_tools_dcm_value::ReadDicomValue<dicom_object::InMemDicomObject>>::read_value(backend)?
                });
            }
        }
    }

    let expanded = quote! {
         impl rad_tools_dcm_value::ReadDicomValue<dicom_object::InMemDicomObject> for #name {
            fn read_value(backend: &dicom_object::InMemDicomObject) -> Result<Self, rad_tools_dcm_value::Error> {
                Ok(
                    Self {
                        #(#read_values),*
                    }
                )
            }
        }
    };
    TokenStream::from(expanded)
}
