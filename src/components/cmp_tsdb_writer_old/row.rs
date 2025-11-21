//! Модель строки в БД

use std::borrow::Cow;

use sqlx::types::time::OffsetDateTime;
use time::format_description::well_known::Iso8601;

use super::Error;

// // ANCHOR: Row
// /// Модель строки в БД
// #[derive(Debug, FromRow)]
// pub struct Row {
//     /// Метка времени
//     pub time: OffsetDateTime,

//     /// Проект
//     pub prj: Cow<'static, str>,

//     /// Хост
//     pub hst: Cow<'static, str>,

//     /// Сервис
//     pub svc: Cow<'static, str>,

//     /// Компонент
//     pub cmp: Cow<'static, str>,

//     /// Ключ
//     pub key: Cow<'static, str>,

//     /// Значение
//     pub value: f64,
// }
// // ANCHOR: Row

/// Построитель записи в БД
pub struct RowBuilder {}
impl RowBuilder {
    /// Добавить название проекта
    pub fn prj(prj: impl Into<Cow<'static, str>>) -> RowPrj {
        RowPrj { prj: prj.into() }
    }
}

/// Информация о проекте
pub struct RowPrj {
    prj: Cow<'static, str>,
}
impl RowPrj {
    /// Добавить название хоста
    pub fn hst(self, hst: impl Into<Cow<'static, str>>) -> RowPrjHst {
        RowPrjHst {
            prj: self.prj,
            hst: hst.into(),
        }
    }
}

/// Информация о проекте и хосте
pub struct RowPrjHst {
    prj: Cow<'static, str>,
    hst: Cow<'static, str>,
}
impl RowPrjHst {
    /// Добавить название сервиса
    pub fn svc(self, svc: impl Into<Cow<'static, str>>) -> RowPrjHstSvc {
        RowPrjHstSvc {
            prj: self.prj,
            hst: self.hst,
            svc: svc.into(),
        }
    }
}

/// Информация о проекте, хосте и сервисе
pub struct RowPrjHstSvc {
    prj: Cow<'static, str>,
    hst: Cow<'static, str>,
    svc: Cow<'static, str>,
}
impl RowPrjHstSvc {
    /// Добавить название компонента
    pub fn cmp(self, cmp: impl Into<Cow<'static, str>>) -> RowPrjHstSvcCmp {
        RowPrjHstSvcCmp {
            prj: self.prj,
            hst: self.hst,
            svc: self.svc,
            cmp: cmp.into(),
        }
    }

    /// Подготовить запись в БД с заданной меткой времени
    pub fn row_with_ts(
        &self,
        cmp: impl AsRef<str>,
        key: impl AsRef<str>,
        value: impl Into<f64>,
        time: &OffsetDateTime,
    ) -> Result<String, Error> {
        let time = time.format(&Iso8601::DEFAULT)?;
        let sql = row_to_sql(
            &time,
            &self.prj,
            &self.hst,
            &self.svc,
            cmp.as_ref(),
            key.as_ref(),
            value.into(),
        );
        Ok(sql)
    }

    /// Подготовить запись в БД с текущим временем
    pub fn row_without_ts(
        &self,
        cmp: impl AsRef<str>,
        key: impl AsRef<str>,
        value: impl Into<f64>,
    ) -> Result<String, Error> {
        let time = OffsetDateTime::now_local()?.format(&Iso8601::DEFAULT)?;
        let sql = row_to_sql(
            &time,
            &self.prj,
            &self.hst,
            &self.svc,
            cmp.as_ref(),
            key.as_ref(),
            value.into(),
        );
        Ok(sql)
    }
}

/// Информация о проекте, хосте, сервисе и компоненте
pub struct RowPrjHstSvcCmp {
    prj: Cow<'static, str>,
    hst: Cow<'static, str>,
    svc: Cow<'static, str>,
    cmp: Cow<'static, str>,
}
impl RowPrjHstSvcCmp {
    /// Подготовить запись в БД с заданной меткой времени
    pub fn row_with_ts(
        &self,
        key: impl AsRef<str>,
        value: impl Into<f64>,
        time: &OffsetDateTime,
    ) -> Result<String, Error> {
        let time = time.format(&Iso8601::DEFAULT)?;
        let sql = row_to_sql(
            &time,
            &self.prj,
            &self.hst,
            &self.svc,
            &self.cmp,
            key.as_ref(),
            value.into(),
        );
        Ok(sql)
    }

    /// Подготовить запись в БД с текущим временем
    pub fn row_without_ts(
        &self,
        key: impl AsRef<str>,
        value: impl Into<f64>,
    ) -> Result<String, Error> {
        let time = OffsetDateTime::now_local()?.format(&Iso8601::DEFAULT)?;
        let sql = row_to_sql(
            &time,
            &self.prj,
            &self.hst,
            &self.svc,
            &self.cmp,
            key.as_ref(),
            value.into(),
        );
        Ok(sql)
    }
}

fn row_to_sql(
    time: &str,
    prj: &str,
    hst: &str,
    svc: &str,
    cmp: &str,
    key: &str,
    value: f64,
) -> String {
    format!(
        "('{}', '{}', '{}', '{}', '{}', '{}', {})",
        time, prj, hst, svc, cmp, key, value
    )
}
