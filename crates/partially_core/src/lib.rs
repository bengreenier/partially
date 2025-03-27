#[cfg(feature = "codegen")]
pub mod codegen;
mod macros;
pub mod prelude;

use derive_builder::Builder;

string_proxy!(Type);
string_proxy!(FieldName);
string_proxy!(StructName);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Visibility {
    #[default]
    Private,
    Pub,
    PubCrate,
}

#[derive(Debug, Builder, Clone, PartialEq, Eq)]
#[builder(setter(prefix = "with"))]
pub struct GenerationConfig {
    #[builder(default, setter(strip_option, name = "with_crate"))]
    pub(crate) krate: Option<String>,
    pub(crate) structs: Vec<StructConfig>,
}

impl GenerationConfig {
    pub fn builder() -> GenerationConfigBuilder {
        GenerationConfigBuilder::default()
    }
}

impl GenerationConfigBuilder {
    pub fn with_struct(mut self, struct_config: StructConfig) -> Self {
        match &mut self.structs {
            Some(structs) => structs.push(struct_config),
            None => self.structs = Some(vec![struct_config]),
        }

        self
    }
}

#[derive(Debug, Builder, Clone, PartialEq, Eq)]
pub struct StructGeneric {
    #[builder(setter(name = "with_type"))]
    pub(crate) ty: String,
    #[builder(default, setter(into, strip_option, name = "with_impl"))]
    pub(crate) imp: Option<String>,
    #[builder(default, setter(into, strip_option, name = "with_where"))]
    pub(crate) wher: Option<String>,
}

impl StructGeneric {
    pub const DEFAULT: Self = Self {
        ty: String::new(),
        imp: None,
        wher: None,
    };

    pub fn builder() -> StructGenericBuilder {
        StructGenericBuilder::default()
    }
}

#[derive(Debug, Builder, Clone, PartialEq, Eq)]
#[builder(setter(prefix = "with"))]
pub struct StructConfig {
    #[builder(setter(into))]
    pub(crate) name: StructName,
    #[builder(default)]
    pub(crate) visibility: Visibility,
    pub(crate) generic: Option<StructGeneric>,
    pub(crate) fields: Vec<FieldConfig>,
    #[builder(default = true)]
    pub(crate) detailed_reporting: bool,
    #[builder(setter(into))]
    pub(crate) derives: Vec<Type>,
    pub(crate) source_attributes: Vec<String>,
    pub(crate) additional_attributes: Vec<String>,
    #[builder(default, setter(into, strip_option))]
    pub(crate) target_name: Option<StructName>,
    #[builder(setter(into))]
    pub(crate) skipped_fields: Vec<FieldName>,
}

impl StructConfig {
    pub fn builder() -> StructConfigBuilder {
        StructConfigBuilder::default()
    }

    pub(crate) fn safe_target_name(&self) -> &StructName {
        &self.target_name.as_ref().unwrap_or_else(|| &self.name)
    }
}

impl StructConfigBuilder {
    pub fn with_field(mut self, field: FieldConfig) -> Self {
        match &mut self.fields {
            Some(fields) => fields.push(field),
            None => self.fields = Some(vec![field]),
        }

        self
    }

    pub fn with_derive(mut self, derive: Type) -> Self {
        match &mut self.derives {
            Some(derives) => derives.push(derive),
            None => self.derives = Some(vec![derive]),
        }

        self
    }

    pub fn with_additional_attribute(mut self, attribute: String) -> Self {
        match &mut self.additional_attributes {
            Some(attributes) => attributes.push(attribute),
            None => self.additional_attributes = Some(vec![attribute]),
        }

        self
    }

    pub fn with_skipped_field<F: Into<FieldName>>(mut self, field: F) -> Self {
        let field = field.into();

        match &mut self.skipped_fields {
            Some(fields) => fields.push(field),
            None => self.skipped_fields = Some(vec![field]),
        }

        self
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FieldFlag {
    #[default]
    None,
    Transparent,
    Into,
}

#[derive(Debug, Builder, Clone, PartialEq, Eq)]
#[builder(setter(prefix = "with"))]
pub struct FieldConfig {
    #[builder(setter(into))]
    pub(crate) name: FieldName,
    #[builder(default)]
    pub(crate) visibility: Visibility,
    #[builder(setter(into, name = "with_type"))]
    pub(crate) ty: Type,
    #[builder(default)]
    pub(crate) flag: FieldFlag,
    #[builder(default, setter(into, strip_option))]
    pub(crate) target_name: Option<FieldName>,
    #[builder(default, setter(into, strip_option, name = "with_target_type"))]
    pub(crate) target_ty: Option<Type>,
}

impl FieldConfig {
    pub fn builder() -> FieldConfigBuilder {
        FieldConfigBuilder::default()
    }
}
