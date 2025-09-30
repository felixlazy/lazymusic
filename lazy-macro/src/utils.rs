use syn::{
    Data::Struct, DataStruct, DeriveInput, Error, Field, Fields::Named, Meta, Result, Token, Type,
    punctuated::Punctuated, spanned::Spanned,
};

/// 从派生宏输入中提取具名字段（Named Fields）
///
/// # 参数
/// - `derive`：派生宏输入的 AST，类型为 `&DeriveInput`
///
/// # 返回
/// - 成功返回字段集合 `&Punctuated<Field, syn::Token![,]>`
/// - 如果不是具名字段结构体，则返回 `syn::Error`
pub(crate) fn extract_named_field(
    derive: &DeriveInput,
) -> Result<&Punctuated<Field, syn::Token![,]>> {
    // 先匹配 derive.data 是否是结构体类型
    if let Struct(DataStruct {
        // 匹配结构体的字段类型是否是具名字段（Named）
        fields: Named(syn::FieldsNamed { named, .. }),
        .. // 忽略其他字段（比如 struct 的属性、生成器等）
    }) = &derive.data
    {
        // 成功匹配，返回字段集合 named
        Ok(named)
    } else {
        // 如果不是具名字段结构体（可能是元组结构体或单元结构体）
        // 使用 derive.span() 定位错误，返回编译错误信息
        Err(syn::Error::new(derive.span(), "不是具名字段结构体"))
    }
}

pub(crate) fn get_field_attribute_args(
    field: &Field,
    name: impl AsRef<str>,
) -> Result<Punctuated<Meta, Token![,]>> {
    for attr in &field.attrs {
        if attr.path().is_ident(name.as_ref()) {
            let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
            return Ok(nested);
        }
    }
    Err(Error::new_spanned(field, "没有属性"))
}
/// 检查字段类型是否属于给定类型名集合
pub(crate) fn has_field_ty(ty: &syn::Type, ty_names: &[&str]) -> bool {
    if let syn::Type::Path(path) = ty
        && let Some(seg) = path.path.segments.last()
    {
        return ty_names.iter().any(|name| seg.ident == *name);
    }
    false
}
