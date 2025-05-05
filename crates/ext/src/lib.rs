use std::path::Path;
mod doctor_diagnostic;

pub use doctor_diagnostic::*;

pub trait PathExt {
  fn to_string_owned(&self) -> String;
}

impl PathExt for Path {
  fn to_string_owned(&self) -> String {
    self.to_string_lossy().to_string()
  }
}

pub trait ValidatorErrorExt {
  fn to_name(&self) -> String;
  fn to_description(&self) -> String;
}

/// A trait for types that can validate configuration files or other resources
///
/// # Examples
///
/// ```rust
/// use doctor_ext::Validator;
/// use doctor_ext::ValidatorErrorExt;
///
/// struct MyValidator;
///
/// struct MyError;
///
/// impl ValidatorErrorExt for MyError {
///     fn to_name(&self) -> String {
///         "MyError".to_string()
///     }
///
///     fn to_description(&self) -> String {
///         "MyError description".to_string()
///     }
/// }
///
/// impl Validator for MyValidator {
///     type Error = MyError;
///
///     fn validate(&self) -> Result<(), Self::Error> {
///         // Validation logic here
///         Ok(())
///     }
/// }
/// ```
pub trait Validator {
  type Error: ValidatorErrorExt;
  fn validate(&self) -> Result<(), Self::Error>;
}

#[macro_export]
macro_rules! define_errors {
    // 添加错误枚举类型名称作为参数
    ($error_enum:ident {
        $(
            #[message = $message:expr]
            $name:ident {
                $(
                    $field:ident: $type:ty
                ),*
                $(,)?
            }
        ),*
        $(,)?
    }) => {
        // 定义错误枚举
        #[derive(thiserror::Error, Debug)]
        pub enum $error_enum {
            $(
                #[error("{}",.0)]
                $name($name),
            )*
        }

        impl doctor_ext::ValidatorErrorExt for $error_enum {
            fn to_name(&self) -> String {
                match self {
                    $(
                        Self::$name { .. } => format!("{}-{}",stringify!($error_enum).to_string(),stringify!($name).to_string()),
                    )*
                }
            }

            fn to_description(&self) -> String {
                self.to_string()
            }
        }

        $(
            // 为每个具体错误类型定义结构体
            #[derive(Debug, typed_builder::TypedBuilder)]
            pub struct $name {
                $(
                    pub $field: $type
                ),*
            }

            impl $name {
                pub fn to_name(&self) -> String {
                    stringify!($name).to_string()
                }
            }

            // 实现 Into trait
            impl Into<$error_enum> for $name {
                fn into(self) -> $error_enum {
                    $error_enum::$name(self)
                }
            }

            // 实现 Result 的 From trait
            impl<T> From<$name> for Result<T, $error_enum> {
                fn from(err: $name) -> Self {
                    Err(err.into())
                }
            }

            // 实现 Display trait
            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, $message $(, $field = self.$field)*)
                }
            }
        )*
    };
}
