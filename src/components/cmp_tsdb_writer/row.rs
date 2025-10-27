//! Модель строки в БД

use std::borrow::Cow;

use sqlx::{FromRow, types::time::OffsetDateTime};

use super::Error;

// ANCHOR: Row
/// Модель строки в БД
#[derive(Debug, FromRow)]
pub struct Row {
    /// Метка времени
    pub time: OffsetDateTime,

    /// Проект
    pub prj: Cow<'static, str>,

    /// Хост
    pub hst: Cow<'static, str>,

    /// Сервис
    pub svc: Cow<'static, str>,

    /// Компонент
    pub cmp: Cow<'static, str>,

    /// Ключ
    pub key: Cow<'static, str>,

    /// Значение
    pub value: f64,
}
// ANCHOR: Row

/// Построитель строки в БД
#[derive(Default, Clone)]
pub struct RowBuilder {
    /// Метка времени
    time: Option<OffsetDateTime>,

    /// Проект
    prj: Option<Cow<'static, str>>,

    /// Хост
    hst: Option<Cow<'static, str>>,

    /// Сервис
    svc: Option<Cow<'static, str>>,

    /// Компонент
    cmp: Option<Cow<'static, str>>,

    /// Ключ
    key: Option<Cow<'static, str>>,

    /// Значение
    value: Option<f64>,
}

impl RowBuilder {
    /// Создать построителя
    pub fn new() -> Self {
        Self::default()
    }

    /// Добавить проект
    pub fn prj(self, prj: impl Into<Cow<'static, str>>) -> Self {
        Self {
            prj: Some(prj.into()),
            ..self
        }
    }

    /// Добавить хост
    pub fn hst(self, hst: impl Into<Cow<'static, str>>) -> Self {
        Self {
            hst: Some(hst.into()),
            ..self
        }
    }

    /// Добавить сервис
    pub fn svc(self, svc: impl Into<Cow<'static, str>>) -> Self {
        Self {
            svc: Some(svc.into()),
            ..self
        }
    }

    /// Добавить компонент
    pub fn cmp(self, cmp: impl Into<Cow<'static, str>>) -> Self {
        Self {
            cmp: Some(cmp.into()),
            ..self
        }
    }

    /// Добавить ключ
    pub fn key(self, key: impl Into<Cow<'static, str>>) -> Self {
        Self {
            key: Some(key.into()),
            ..self
        }
    }

    /// Добавить значение
    pub fn value(self, value: impl Into<f64>) -> Self {
        Self {
            value: Some(value.into()),
            ..self
        }
    }

    /// Добавить время
    pub fn time(self, time: impl Into<OffsetDateTime>) -> Self {
        Self {
            time: Some(time.into()),
            ..self
        }
    }

    /// Собрать строку
    pub fn row(self) -> Result<Row, Error> {
        let prj = match self.prj {
            Some(v) => v,
            None => {
                let err = "Empty field 'prj'".to_string();
                return Err(Error::RowIncorrect(err));
            }
        };

        let hst = match self.hst {
            Some(v) => v,
            None => {
                let err = "Empty field 'hst'".to_string();
                return Err(Error::RowIncorrect(err));
            }
        };

        let svc = match self.svc {
            Some(v) => v,
            None => {
                let err = "Empty field 'svc'".to_string();
                return Err(Error::RowIncorrect(err));
            }
        };

        let cmp = match self.cmp {
            Some(v) => v,
            None => {
                let err = "Empty field 'cmp'".to_string();
                return Err(Error::RowIncorrect(err));
            }
        };

        let key = match self.key {
            Some(v) => v,
            None => {
                let err = "Empty field 'key'".to_string();
                return Err(Error::RowIncorrect(err));
            }
        };

        let value = match self.value {
            Some(v) => v,
            None => {
                let err = "Empty field 'value'".to_string();
                return Err(Error::RowIncorrect(err));
            }
        };

        let time = match self.time {
            Some(v) => v,
            None => OffsetDateTime::now_utc(),
        };

        Ok(Row {
            time,
            prj,
            hst,
            svc,
            cmp,
            key,
            value,
        })
    }
}
