use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Type, parse_macro_input};

/// Derive macro to automatically implement the [`ScreenState`](ratapp::ScreenState) trait for an
/// enum representing the application's screens.
///
/// Each variant of the enum should hold a single unnamed field of the screen type. For example:
///
/// ```ignore
/// #[derive(ratapp::Screens)]
/// enum AppScreens {
///     Home(HomeScreen),
///     Settings(SettingsScreen),
/// }
/// ```
///
/// This macro will generate:
///
/// - A `ScreenID` enum with variants corresponding to each screen.
/// - An implementation of the `ScreenState` trait for the enum, forwarding method calls to the
///   active screen.
///
/// To learn how to implement screen state without this macro, check out the
/// [`ScreenState`](ratapp::ScreenState) trait documentation.
#[proc_macro_derive(Screens)]
pub fn screen(input: proc_macro::TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match screens_derive(&input) {
        Ok(tokens) => tokens,
        Err(tokens) => tokens,
    }
}

fn screens_derive(input: &DeriveInput) -> Result<TokenStream, TokenStream> {
    let r#enum = get_enum(input)?;
    let variants = get_screens_variants(r#enum)?;

    let screen_id_tokens = generate_screen_id(&variants);
    let screen_state_impl = generate_screen_state_impl(&input.ident, &variants);

    Ok(quote! {
        #screen_id_tokens

        #screen_state_impl
    }
    .into())
}

fn get_enum(input: &DeriveInput) -> Result<&syn::DataEnum, proc_macro::TokenStream> {
    match &input.data {
        Data::Enum(data_enum) => Ok(data_enum),
        _ => Err(quote! {
            compile_error!("#[derive(ratapp::Screens)] can only be used on enums. Check out the ratapp documentation for more information.");
        }
        .into()),
    }
}

fn get_screens_variants(input: &DataEnum) -> Result<Vec<(&Ident, &Type)>, proc_macro::TokenStream> {
    let mut result = Vec::new();

    for variant in &input.variants {
        let name = &variant.ident;
        let ty = match &variant.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => &fields.unnamed[0].ty,
            _ => {
                return Err(quote! {
                    compile_error!("#[derive(ratapp::Screens)] can only be used on enums with single unnamed field variants (i.e. `Variant(YourScreenType)`). Check out the ratapp documentation for more information.");
                }.into());
            }
        };
        result.push((name, ty));
    }

    Ok(result)
}

// TODO: Base `pub` on app's `Screen` enum visibility.
fn generate_screen_id(variants: &[(&Ident, &Type)]) -> proc_macro2::TokenStream {
    let ids = variants.iter().map(|(name, _)| name);

    quote! {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum ScreenID {
            #(#ids),*
        }
    }
}

fn generate_screen_state_impl(
    enum_name: &Ident,
    variants: &[(&Ident, &Type)],
) -> proc_macro2::TokenStream {
    let where_bounds = variants.iter().map(|(_, ty)| {
        quote! {
            #ty : ratapp::ScreenWithState<ScreenID, S>
        }
    });

    let match_new = variants.iter().map(|(name, ty)| {
        quote! {
            ScreenID::#name => #enum_name::#name(#ty::default()),
        }
    });

    let match_draw = variants.iter().map(|(name, _)| {
        quote! {
            #enum_name::#name(screen) => ScreenWithState::draw(screen, frame, state),
        }
    });

    let match_on_event = variants.iter().map(|(name, _)| {
        quote! {
            #enum_name::#name(screen) => ScreenWithState::on_event(screen, event, navigator, state).await,
        }
    });

    let match_on_enter = variants.iter().map(|(name, _)| {
        quote! {
            #enum_name::#name(screen) => ScreenWithState::on_enter(screen, navigator, state).await,
        }
    });

    let match_on_exit = variants.iter().map(|(name, _)| {
        quote! {
            #enum_name::#name(screen) => ScreenWithState::on_exit(screen, navigator, state).await,
        }
    });

    let match_on_pause = variants.iter().map(|(name, _)| {
        quote! {
            #enum_name::#name(screen) => ScreenWithState::on_pause(screen, navigator, state).await,
        }
    });

    let match_on_resume = variants.iter().map(|(name, _)| {
        quote! {
            #enum_name::#name(screen) => ScreenWithState::on_resume(screen, navigator, state).await,
        }
    });

    let screen_state_impl = quote! {
        impl<S> ratapp::ScreenState<S> for #enum_name
        where
            #( #where_bounds, )*
        {
            type ID = ScreenID;

            fn new(id: Self::ID) -> Self {
                match id {
                    #(#match_new)*
                }
            }

            fn draw(&mut self, frame: &mut ratatui::Frame, state: &S) {
                use ratapp::ScreenWithState;

                match self {
                    #(#match_draw)*
                }
            }

            async fn on_event(&mut self, event: ratatui::crossterm::event::Event, navigator: ratapp::Navigator<Self::ID>, state: &mut S) {
                use ratapp::ScreenWithState;

                match self {
                    #(#match_on_event)*
                }
            }

            async fn on_enter(&mut self, navigator: ratapp::Navigator<Self::ID>, state: &mut S) {
                use ratapp::ScreenWithState;

                match self {
                    #(#match_on_enter)*
                }
            }

            async fn on_exit(&mut self, navigator: ratapp::Navigator<Self::ID>, state: &mut S) {
                use ratapp::ScreenWithState;

                match self {
                    #(#match_on_exit)*
                }
            }

            async fn on_pause(&mut self, navigator: ratapp::Navigator<Self::ID>, state: &mut S) {
                use ratapp::ScreenWithState;

                match self {
                    #(#match_on_pause)*
                }
            }

            async fn on_resume(&mut self, navigator: ratapp::Navigator<Self::ID>, state: &mut S) {
                use ratapp::ScreenWithState;

                match self {
                    #(#match_on_resume)*
                }
            }
        }
    };

    screen_state_impl
}
