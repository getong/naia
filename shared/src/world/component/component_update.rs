use std::collections::HashSet;

use naia_serde::{BitReader, OwnedBitReader};

use crate::{
    world::component::component_kinds::ComponentKind, ComponentKinds, LocalEntity,
    LocalEntityAndGlobalEntityConverter,
};

pub struct ComponentUpdate {
    pub kind: ComponentKind,
    buffer: OwnedBitReader,
}

impl ComponentUpdate {
    pub fn new(kind: ComponentKind, buffer: OwnedBitReader) -> Self {
        Self { kind, buffer }
    }

    pub fn reader(&self) -> BitReader {
        self.buffer.borrow()
    }

    pub(crate) fn split_into_waiting_and_ready(
        self,
        converter: &dyn LocalEntityAndGlobalEntityConverter,
        component_kinds: &ComponentKinds,
    ) -> (Option<(HashSet<LocalEntity>, Self)>, Option<Self>) {
        todo!()
    }
}
