

pub trait ReflectStructMacro<'a, const LENGTH: usize> {
    fn list_fields_names() -> [&'a str; LENGTH];
    fn fields_count() -> usize;
}
