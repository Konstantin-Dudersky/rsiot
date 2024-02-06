use formatx::formatx;

pub trait IMsgContentValue {
    /// Форматирование значения c заданным форматом
    fn fmt_value(&self, template: &str) -> String;
}

/// Форматирование примитивных типов с помощью библиотеки `formatx`. Есть ограничения. Возможно
/// найдется более универсальный вариант
fn format_with_formatx<TValue>(value: &TValue, template: &str) -> String
where
    TValue: std::fmt::Debug + std::fmt::Display,
{
    let res = formatx!(template, value);
    match res {
        Ok(val) => val,
        Err(err) => err.to_string(),
    }
}

impl IMsgContentValue for () {
    fn fmt_value(&self, _template: &str) -> String {
        String::from("")
    }
}

impl IMsgContentValue for bool {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for std::time::Duration {
    fn fmt_value(&self, _template: &str) -> String {
        format!("{:?}", self)
    }
}

impl IMsgContentValue for f32 {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for f64 {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for i8 {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for i16 {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for i32 {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for i64 {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for i128 {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for isize {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for rgb::RGB8 {
    fn fmt_value(&self, _template: &str) -> String {
        format!("{}", self)
    }
}

impl IMsgContentValue for String {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for u8 {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for u16 {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for u32 {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for u64 {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for u128 {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}

impl IMsgContentValue for usize {
    fn fmt_value(&self, template: &str) -> String {
        format_with_formatx(self, template)
    }
}
