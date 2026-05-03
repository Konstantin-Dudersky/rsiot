use esp_idf_svc::hal::can::config::Filter;

use crate::components_config::can_general::CanFilter;

use super::Error;

const ERROR_TEXT: &str = r#"Unsupported filter setting. There are three possible options:
- One Standard filter
- One Extended Filter
- Two Standard filters"#;
const ERROR: Result<Filter, Error> = Err(Error::FilterSetup(ERROR_TEXT));

pub fn can_filter_convert(filter: &[CanFilter]) -> Result<Filter, Error> {
    let filter = match filter.len() {
        0 => Filter::Standard { filter: 0, mask: 0 },

        1 => match filter[0] {
            CanFilter::Standard { id: filter, mask } => Filter::Standard { filter, mask },
            CanFilter::Extended { id: filter, mask } => Filter::Extended { filter, mask },
            _ => return ERROR,
        },

        2 => match (filter[0], filter[1]) {
            (
                CanFilter::Standard {
                    id: filter1,
                    mask: mask1,
                },
                CanFilter::Standard {
                    id: filter2,
                    mask: mask2,
                },
            ) => Filter::Dual {
                filter1,
                mask1,
                filter2,
                mask2,
            },

            _ => return ERROR,
        },

        _ => return ERROR,
    };
    Ok(filter)
}
