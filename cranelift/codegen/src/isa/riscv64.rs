use crate::isa::riscv_shared;
use crate::isa::riscv_shared::settings as riscv_settings;
use crate::isa::Builder as IsaBuilder;
use target_lexicon::{Architecture, PointerWidth, Triple};

/// Create a new `isa::Builder`.
pub fn isa_builder(triple: Triple) -> IsaBuilder {
    match triple.architecture {
        Architecture::Riscv64(..) => {}
        _ => unreachable!(),
    }
    IsaBuilder {
        triple,
        setup: riscv_settings::builder(),
        constructor: |triple, shared_flags, builder| {
            riscv_shared::isa_constructor(triple, shared_flags, builder, PointerWidth::U64)
        },
    }
}
