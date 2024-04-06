use std::convert::Infallible;

use rkyv::ser::serializers::{
    AllocScratchError, CompositeSerializerError, SharedSerializeMapError,
};

#[rustfmt::skip]
pub type CompositeSerializer = CompositeSerializerError<Infallible, AllocScratchError, SharedSerializeMapError>;
