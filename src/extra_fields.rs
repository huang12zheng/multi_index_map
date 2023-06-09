use proc_macro2::TokenStream;

use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_quote, token, Attribute, Field, Ident, Path, Token, Type, Visibility,
};

struct AttrInner {
    _paren_token: token::Paren,
    pub path: Path,
    pub tokens: TokenStream,
}
impl Parse for AttrInner {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        Ok(Self {
            _paren_token: parenthesized!(content in input),
            path: content.call(Path::parse_mod_style)?,
            tokens: content.parse()?,
        })
    }
}
struct ExtraFiledAst {
    _paren_token: token::Paren,
    pub directly: Ident,
    _s1: Token![,],
    pub func: Ident,
    _s2: Token![,],
    pub ty: Type,
}
impl Parse for ExtraFiledAst {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        Ok(Self {
            _paren_token: parenthesized!(content in input),
            directly: content.parse()?,
            _s1: content.parse()?,
            func: content.parse()?,
            _s2: content.parse()?,
            ty: content.parse()?,
        })
    }
}

pub trait IntoExtraFiled {
    type Error;
    fn into_extra_filed(&self, vis: Visibility) -> Result<Option<Field>, Self::Error>;
}
// impl TryFrom<Attribute> for Option<ExtraField> {
impl IntoExtraFiled for Attribute {
    fn into_extra_filed(&self, vis: Visibility) -> Result<Option<Field>, Self::Error> {
        if self.path.is_ident("multi_index") {
            let tokens = &self.tokens;
            let inner: AttrInner = parse_quote!(#tokens);
            if inner.path.is_ident("extra_field") {
                let tokens = &inner.tokens;
                let ExtraFiledAst {
                    directly,
                    func: ident,
                    ty,
                    ..
                } = parse_quote!(#tokens);
                let attr: Attribute = parse_quote!(#[multi_index(#directly)] );
                let ident = Some(ident);
                return Ok(Some(Field {
                    attrs: vec![attr],
                    vis: vis,
                    ident,
                    colon_token: None,
                    ty,
                }));
            }
        }
        Ok(None)
    }

    type Error = syn::Error;
}
