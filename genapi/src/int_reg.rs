use super::{
    elem_type::{Endianness, IntegerRepresentation, Sign},
    interface::{IInteger, IRegister, ISelector, IncrementMode},
    node_base::{NodeAttributeBase, NodeBase},
    register_base::RegisterBase,
    store::{CacheStore, NodeId, NodeStore, ValueStore},
    Device, GenApiResult, ValueCtxt,
};

#[derive(Debug, Clone)]
pub struct IntRegNode {
    pub(crate) attr_base: NodeAttributeBase,
    pub(crate) register_base: RegisterBase,

    pub(crate) sign: Sign,
    pub(crate) endianness: Endianness,
    pub(crate) unit: Option<String>,
    pub(crate) representation: IntegerRepresentation,
    pub(crate) p_selected: Vec<NodeId>,
}

impl IntRegNode {
    #[must_use]
    pub fn node_base(&self) -> NodeBase {
        let elem_base = &self.register_base.elem_base;
        NodeBase::new(&self.attr_base, elem_base)
    }

    #[must_use]
    pub fn register_base(&self) -> &RegisterBase {
        &self.register_base
    }

    #[must_use]
    pub fn sign(&self) -> Sign {
        self.sign
    }

    #[must_use]
    pub fn endianness(&self) -> Endianness {
        self.endianness
    }

    #[must_use]
    pub fn unit_elem(&self) -> Option<&str> {
        self.unit.as_deref()
    }

    #[must_use]
    pub fn representation_elem(&self) -> IntegerRepresentation {
        self.representation
    }

    #[must_use]
    pub fn p_selected(&self) -> &[NodeId] {
        &self.p_selected
    }
}

impl IInteger for IntRegNode {
    fn value<T: ValueStore, U: CacheStore>(
        &self,
        device: &mut impl Device,
        store: &impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<i64> {
        todo!()
    }

    fn set_value<T: ValueStore, U: CacheStore>(
        &self,
        value: i64,
        device: &mut impl Device,
        store: &impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<()> {
        todo!()
    }

    fn min<T: ValueStore, U: CacheStore>(
        &self,
        device: &mut impl Device,
        store: &impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<i64> {
        Ok(i64::MIN)
    }

    fn max<T: ValueStore, U: CacheStore>(
        &self,
        device: &mut impl Device,
        store: &impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<i64> {
        Ok(i64::MAX)
    }

    fn inc_mode(&self, store: &impl NodeStore) -> GenApiResult<Option<IncrementMode>> {
        Ok(None)
    }

    fn inc<T: ValueStore, U: CacheStore>(
        &self,
        device: &mut impl Device,
        store: &impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<Option<i64>> {
        Ok(None)
    }

    fn valid_value_set(&self, store: &impl NodeStore) -> &[i64] {
        &[]
    }

    fn representation(&self, store: &impl NodeStore) -> IntegerRepresentation {
        todo!()
    }

    fn unit(&self, store: &impl NodeStore) -> Option<&str> {
        todo!()
    }

    fn set_min<T: ValueStore, U: CacheStore>(
        &self,
        value: i64,
        device: &mut impl Device,
        store: &impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<()> {
        Ok(())
    }

    fn set_max<T: ValueStore, U: CacheStore>(
        &self,
        _: i64,
        _: &mut impl Device,
        _: &impl NodeStore,
        _: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<()> {
        Ok(())
    }
}

impl IRegister for IntRegNode {
    fn read<T: ValueStore, U: CacheStore>(
        &self,
        buf: &mut [u8],
        device: &mut impl Device,
        store: &impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<()> {
        self.register_base().read(buf, device, store, cx)
    }

    fn write<T: ValueStore, U: CacheStore>(
        &self,
        buf: &[u8],
        device: &mut impl Device,
        store: &impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<()> {
        self.register_base().write(buf, device, store, cx)
    }

    fn address<T: ValueStore, U: CacheStore>(
        &self,
        device: &mut impl Device,
        store: &impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<i64> {
        self.register_base().address(device, store, cx)
    }

    fn length<T: ValueStore, U: CacheStore>(
        &self,
        device: &mut impl Device,
        store: &impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<i64> {
        self.register_base().length(device, store, cx)
    }
}

impl ISelector for IntRegNode {
    fn selecting_nodes(&self, store: &impl NodeStore) -> GenApiResult<&[NodeId]> {
        Ok(self.p_selected())
    }
}
