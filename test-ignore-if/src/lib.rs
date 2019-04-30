extern crate proc_macro;

use proc_macro::TokenStream;

use darling::FromMeta;
use quote::quote;
use syn::AttributeArgs;

#[derive(FromMeta)]
struct Attrs {
    env_set: String,
    #[darling(default)]
    enabled: Option<bool>,
}

#[proc_macro_attribute]
pub fn ignore_if(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as AttributeArgs);

    let attr = match Attrs::from_list(&attr) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    if !std::env::var(format!(
        "TEST_IGNORE_IF_ENV_SET_ENABLED_FOR_{}",
        attr.env_set
    ))
    .is_ok()
    {
        panic!(format!(
            "trying to use ignore_if_env_set for env var \"{}\" that is not enabled, you \
             need to add test_ignore_if_utils::enable_ignore_if_env_set_for(\"{}\") in your \
             build.rs",
            attr.env_set, attr.env_set
        ))
    }

    let item = syn::parse_macro_input!(item);

    if attr.enabled.unwrap_or_else(|| true) && std::env::var(attr.env_set).is_ok() {
        quote! {
            #[ignore]
            #item
        }
    } else {
        item
    }
    .into()
}
