pub trait Event<'a> {
    fn type_name(&self) -> &'a str;
}

pub struct Entry<'a, E: 'a + Event<'a>> {
    position: i64,
    version: i64,
    type_name: &'a str,
    payload: &'a E,
}

impl<'a, E> Entry<'a, E>
where
    E: 'a + Event<'a>,
{
    fn new(position: i64, version: i64, payload: &'a E) -> Entry<'a, E> {
        Entry {
            position: position,
            version: version,
            type_name: payload.type_name(),
            payload: &payload,
        }
    }

    fn payload(&self) -> &E {
        self.payload
    }
}

pub trait Reducer<'a, E>
where
    E: Event<'a>,
{
    fn apply(self, id: &'a str, e: E) -> Self;
}
