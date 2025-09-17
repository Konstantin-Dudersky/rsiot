use super::CanFilter;

impl From<CanFilter> for socketcan::CanFilter {
    fn from(value: CanFilter) -> Self {
        match value {
            CanFilter::Standard { id, mask } => socketcan::CanFilter::new(id.into(), mask.into()),
            CanFilter::StandardInverted { id, mask } => {
                socketcan::CanFilter::new_inverted(id.into(), mask.into())
            }
            CanFilter::Extended { id, mask } => socketcan::CanFilter::new(id, mask),
            CanFilter::ExtendedInverted { id, mask } => {
                socketcan::CanFilter::new_inverted(id, mask)
            }
        }
    }
}
