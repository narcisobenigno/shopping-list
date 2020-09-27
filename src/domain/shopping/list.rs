use crate::es::event;

#[derive(Debug)]
pub struct List<'a> {
    id: &'a str,
    name: &'a str,
}

impl<'a> List<'a> {
    pub fn new() -> List<'a> {
        List { id: "", name: "" }
    }
}

pub struct Created<'a> {
    id: &'a str,
    name: &'a str,
}

impl<'a> Created<'a> {
    fn new(id: &'a str, name: &'a str) -> Self {
        Created { id: id, name: name }
    }
}

impl<'a> event::Event<'a> for Created<'a> {
    fn type_name(&self) -> &'a str {
        std::any::type_name::<Created>()
    }
}

impl<'a> event::Reducer<'a, Created<'a>> for List<'a> {
    fn apply(self, id: &'a str, payload: Created<'a>) -> List<'a> {
        List {
            id: id,
            name: payload.name,
        }
    }
}

pub struct Renamed<'a> {
    name: &'a str,
}

impl<'a> Renamed<'a> {
    fn new(name: &'a str) -> Self {
        Renamed { name: name }
    }
}

impl<'a> event::Event<'a> for Renamed<'a> {
    fn type_name(&self) -> &'a str {
        std::any::type_name::<Renamed<'a>>()
    }
}

impl<'a> event::Reducer<'a, Renamed<'a>> for List<'a> {
    fn apply(self, id: &'a str, payload: Renamed<'a>) -> List<'a> {
        List {
            name: payload.name,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::shopping::list;
    use crate::es::event::Reducer;
    #[test]
    fn it_adds_items() {
        let l = list::List::new()
            .apply(
                "list-uuid-1",
                list::Created::new(&"list-uuid-1", &"List name"),
            )
            .apply("list-uuid-1", list::Renamed::new(&"List new name"));
        println!("{:?}", l);
        assert_eq!(2 + 2, 4);
    }
}
