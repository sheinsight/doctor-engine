use std::path::Path;

pub trait PathExt {
  fn to_string_owned(&self) -> String;
}

impl PathExt for Path {
  fn to_string_owned(&self) -> String {
    self.to_string_lossy().to_string()
  }
}

pub trait MultiFrom: Sized {
  type Error;
  fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error>;
  fn from_cwd<P: AsRef<Path>>(cwd: P) -> Result<Self, Self::Error>;
}

/// A trait for types that can validate configuration files or other resources
///
/// # Examples
///
/// ```rust
/// use doctor_ext::Validator;
///
/// struct MyValidator;
///
/// impl Validator for MyValidator {
///     type Error = std::io::Error;
///
///     fn validate(&self) -> Result<(), Self::Error> {
///         // Validation logic here
///         Ok(())
///     }
/// }
/// ```
pub trait Validator {
  type Error;
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

        $(
            // 为每个具体错误类型定义结构体
            #[derive(Debug, typed_builder::TypedBuilder)]
            pub struct $name {
                $(
                    pub $field: $type
                ),*
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
