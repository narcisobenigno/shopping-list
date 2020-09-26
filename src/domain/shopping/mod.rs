pub trait Event<'a> {
    fn type_name(&self) -> &'a str;
}

pub struct EventEntry<'a, E: 'a + Event<'a>> {
    position: &'a i64,
    version: &'a i64,
    type_name: &'a str,
    payload: &'a E,
}

impl<'a, E> EventEntry<'a, E>
where
    E: 'a + Event<'a>,
{
    fn new(position: &'a i64, version: &'a i64, payload: &'a E) -> EventEntry<'a, E> {
        EventEntry {
            position: &position,
            version: &version,
            type_name: payload.type_name(),
            payload: &payload,
        }
    }

    fn payload(&self) -> &E {
        self.payload
    }
}

trait EventReducer<'a, E>
where
    E: Event<'a>,
{
    fn apply(self, id: &'static str, e: E) -> Self;
}

#[derive(Debug)]
pub struct List {
    id: &'static str,
    name: &'static str,
}

impl List {
    pub fn new() -> List {
        List { id: "", name: "" }
    }
}

pub struct ListCreated {
    id: &'static str,
    name: &'static str,
}

impl ListCreated {
    fn new(id: &'static str, name: &'static str) -> Self {
        ListCreated { id: id, name: name }
    }
}

impl<'a> Event<'a> for ListCreated {
    fn type_name(&self) -> &'a str {
        std::any::type_name::<ListCreated>()
    }
}

impl<'a> EventReducer<'a, ListCreated> for List {
    fn apply(self, id: &'static str, payload: ListCreated) -> List {
        List {
            id: &id,
            name: &payload.name,
        }
    }
}

pub struct ListRenamed {
    name: &'static str,
}

impl ListRenamed {
    fn new(name: &'static str) -> Self {
        ListRenamed { name: name }
    }
}

impl<'a> Event<'a> for ListRenamed {
    fn type_name(&self) -> &'a str {
        std::any::type_name::<ListRenamed>()
    }
}

impl<'a> EventReducer<'a, ListRenamed> for List {
    fn apply(self, id: &'static str, payload: ListRenamed) -> List {
        List {
            name: &payload.name,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::shopping::EventReducer;
    use crate::domain::shopping::List;
    use crate::domain::shopping::ListCreated;
    use crate::domain::shopping::ListRenamed;
    #[test]
    fn it_adds_items() {
        let l = List::new()
            .apply(
                "list-uuid-1",
                ListCreated::new(&"list-uuid-1", &"List name"),
            )
            .apply("list-uuid-1", ListRenamed::new(&"List new name"));
        println!("{:?}", l);
        assert_eq!(2 + 2, 4);
    }
}
