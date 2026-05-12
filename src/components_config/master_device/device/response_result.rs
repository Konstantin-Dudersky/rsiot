/// Результат обработки ответа
#[derive(Clone)]
pub enum ResponseResult {
    /// Запрос успешен - инициализация завершена
    OkInitCompleted,

    /// Запрос успешен - требуется следующий запрос
    OkNeedRequest,

    /// Запрос успешен
    Ok,

    /// Запрос неуспешен
    Error(String),
}

impl ResponseResult {
    /// Запрос успешен - инициализация завершена
    pub fn ok_init_completed<TError>() -> Result<Self, TError> {
        Ok(Self::OkInitCompleted)
    }

    /// Запрос успешен - требуется следующий запрос
    pub fn ok_need_request<TError>() -> Result<Self, TError> {
        Ok(Self::OkNeedRequest)
    }

    /// Запрос успешен
    pub fn ok<TError>() -> Result<Self, TError> {
        Ok(Self::Ok)
    }

    /// Запрос неуспешен
    pub fn error<TError>(msg: String) -> Result<Self, TError> {
        Ok(Self::Error(msg))
    }
}
