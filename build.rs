use std::{collections::HashMap, convert::TryFrom, path::PathBuf, str::FromStr};

use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens, TokenStreamExt};

type E = Box<dyn std::error::Error>;

#[derive(serde::Deserialize)]
struct EndpointDescInternal {
    min: String,
    max: String,
    format: String,
}

#[derive(serde::Deserialize)]
#[serde(try_from = "EndpointDescInternal")]
struct EndpointDesc {
    min: usize,
    max: usize,
    format: String,
    with_padding: usize,
}

impl TryFrom<EndpointDescInternal> for EndpointDesc {
    type Error = <usize as FromStr>::Err;

    fn try_from(d: EndpointDescInternal) -> Result<Self, Self::Error> {
        Ok(Self {
            min: d.min.parse()?,
            max: d.max.parse()?,
            format: d.format,
            with_padding: d.max.len(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), E> {
    let client = reqwest::Client::new();
    #[cfg(feature = "local")]
    {
        use quote::quote;
        let endpoints = get_endpoints_data(&client).await?;

        struct CategoryData<'a>(&'a str, &'a EndpointDesc);

        impl<'a> ToTokens for CategoryData<'a> {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                let name = format_ident!("{}{}", self.0[..1].to_uppercase(), &self.0[1..]);
                let min = self.1.min;
                let max = self.1.max;
                let format = &self.1.format;
                let with_padding = self.1.with_padding;

                tokens.append_all(quote! {
                    pub struct #name;

                    impl super::LocalNekosBestCategory for #name {
                        const CATEGORY: crate::Category = crate::Category::#name;
                        const MIN: usize = #min;
                        const MAX: usize = #max;
                        const WITH_PADDING: usize = #with_padding;
                        const FORMAT: &'static str = #format;
                    }

                    impl #name {
                        pub fn get_random(&self, random: usize) -> String {
                            <Self as super::LocalNekosBestCategory>::get_random(self, random)
                        }
                    
                        pub fn get(&self) -> String {
                            <Self as super::LocalNekosBestCategory>::get(self)
                        }
                    }
                })
            }
        }

        let implementations = endpoints
            .into_iter()
            .map(|(name, endpoint)| {
                let category_data = CategoryData(&name, &endpoint);

                quote! {#category_data}
            })
            .collect::<TokenStream>();

        let implementations = implementations.to_string();

        std::fs::write(PathBuf::from(std::env::var("OUT_DIR")?).join("local_implementation.rs"), implementations)?;
    }

    Ok(())
}

const BASE_URL: &str = "https://nekos.best";

async fn get_endpoints_data(client: &reqwest::Client) -> Result<HashMap<String, EndpointDesc>, E> {
    Ok(client
        .get(format!("{}/endpoints", BASE_URL))
        .send()
        .await?
        .json()
        .await?)
}