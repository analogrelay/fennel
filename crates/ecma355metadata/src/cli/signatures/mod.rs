macro_rules! write_list {
    ($target:expr, $vals:expr, $separator: expr) => {
        let mut any = false;
        for val in $vals {
            any = true;
            write!($target, "{}", val)?;
        }
        if any {
            write!($target, $separator)?;
        }
    };
}

mod custom_modifier;
mod method_signature;
mod param;
mod ret_type;
mod signature_header;
mod type_reference;

pub mod utils;

pub use self::custom_modifier::CustomModifier;
pub use self::method_signature::MethodSignature;
pub use self::param::Param;
pub use self::ret_type::RetType;
pub use self::signature_header::{SignatureAttributes, SignatureCallingConvention, SignatureHeader,
                                 SignatureKind};
pub use self::type_reference::{ArrayShape, TypeReference};
