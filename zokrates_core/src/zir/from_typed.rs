use crate::typed_absy;
use crate::zir;

impl<'ast> From<typed_absy::types::ConcreteFunctionKey<'ast>> for zir::types::FunctionKey<'ast> {
    fn from(k: typed_absy::types::ConcreteFunctionKey<'ast>) -> zir::types::FunctionKey<'ast> {
        zir::types::FunctionKey {
            module: k.module,
            id: k.id,
            signature: k.signature.into(),
        }
    }
}
impl From<typed_absy::types::ConcreteSignature> for zir::types::Signature {
    fn from(s: typed_absy::types::ConcreteSignature) -> zir::types::Signature {
        zir::types::Signature {
            inputs: s.inputs.into_iter().flat_map(|t| from_type(t)).collect(),
            outputs: s.outputs.into_iter().flat_map(|t| from_type(t)).collect(),
        }
    }
}

fn from_type(t: typed_absy::types::ConcreteType) -> Vec<zir::types::Type> {
    match t {
        typed_absy::types::ConcreteType::Int => unreachable!(),
        typed_absy::types::ConcreteType::FieldElement => vec![zir::Type::FieldElement],
        typed_absy::types::ConcreteType::Boolean => vec![zir::Type::Boolean],
        typed_absy::types::ConcreteType::Uint(bitwidth) => {
            vec![zir::Type::uint(bitwidth.to_usize())]
        }
        typed_absy::types::ConcreteType::Array(array_type) => {
            let inner = from_type(*array_type.ty);
            (0..array_type.size).flat_map(|_| inner.clone()).collect()
        }
        typed_absy::types::ConcreteType::Struct(members) => members
            .into_iter()
            .flat_map(|struct_member| from_type(*struct_member.ty))
            .collect(),
    }
}
