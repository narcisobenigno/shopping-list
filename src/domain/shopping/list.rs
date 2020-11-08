use cqrs_es::{Aggregate, AggregateError, Command, DomainEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct List {
    id: String,
    name: String,
}

impl Aggregate for List {
    fn aggregate_type() -> &'static str {
        "shopping.list"
    }
}

impl Default for List {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ListEvent {
    CustomerCreatedList(CustomerCreatedList),
    CustomerRenamedList(CustomerRenamedList),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerCreatedList {
    id: String,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerRenamedList {
    former: String,
    new: String,
}

impl DomainEvent<List> for CustomerCreatedList {
    fn apply(self, aggregate: &mut List) {
        aggregate.id = self.id;
        aggregate.name = self.name
    }
}

impl DomainEvent<List> for CustomerRenamedList {
    fn apply(self, aggregate: &mut List) {
        aggregate.name = self.new;
    }
}

impl DomainEvent<List> for ListEvent {
    fn apply(self, aggregate: &mut List) {
        match self {
            ListEvent::CustomerCreatedList(e) => e.apply(aggregate),
            ListEvent::CustomerRenamedList(e) => e.apply(aggregate),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CustomerCreateList {
    id: String,
    name: String,
}

impl Command<List, ListEvent> for CustomerCreateList {
    fn handle(self, list: &List) -> Result<Vec<ListEvent>, AggregateError> {
        let event = ListEvent::CustomerCreatedList(CustomerCreatedList {
            id: self.id,
            name: self.name,
        });
        Ok(vec![event])
    }
}

#[cfg(test)]
mod tests {
    use cqrs_es::test::TestFramework;

    use crate::domain::shopping;

    type ShoppingListSuite = TestFramework<shopping::list::List, shopping::list::ListEvent>;

    #[test]
    fn it_adds_items() {
        let expected =
            shopping::list::ListEvent::CustomerCreatedList(shopping::list::CustomerCreatedList {
                id: "list-uuid-1".to_string(),
                name: "List name".to_string(),
            });
        ShoppingListSuite::default()
            .given_no_previous_events()
            .when(shopping::list::CustomerCreateList {
                id: "list-uuid-1".to_string(),
                name: "List name".to_string(),
            })
            .then_expect_events(vec![expected]);
    }
}
