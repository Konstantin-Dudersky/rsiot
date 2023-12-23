use rsiot_extra_components::cmp_cache::CacheType;
use rsiot_messages_core::IMessage;

pub struct Config<TMessage>
where
    TMessage: IMessage,
{
    pub cache: CacheType<TMessage>,
}
