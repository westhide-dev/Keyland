use std::convert::Infallible;

use rkyv::ser::serializers::{
    AllocScratchError, CompositeSerializerError, SharedSerializeMapError,
};

#[rustfmt::skip]
pub type RkyvCompositeSerializer = CompositeSerializerError<Infallible, AllocScratchError, SharedSerializeMapError>;
