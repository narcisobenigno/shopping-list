use cqrs_es::{Aggregate, AggregateError, Command, DomainEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct List {
    id: String,
    name: String,
}

impl List {
    fn create(&self, event: CustomerCreateList) -> Result<Vec<ListEvent>, AggregateError> {
        Ok(vec![ListEvent::CustomerCreatedList(CustomerCreatedList {
            id: event.id,
            name: event.name,
        })])
    }

    fn rename(&self, event: CustomerRenameList) -> Result<Vec<ListEvent>, AggregateError> {
        if event.new == self.name {
            return Ok(vec![]);
        }

        Ok(vec![ListEvent::CustomerRenamedList(CustomerRenamedList {
            former: self.name.to_string(),
            new: event.new,
        })])
    }
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

trait ListEventApply<E> {
    fn apply(&mut self, event: E);
}

impl ListEventApply<CustomerCreatedList> for List {
    fn apply(&mut self, event: CustomerCreatedList) {
        self.id = event.id;
        self.name = event.name
    }
}

impl ListEventApply<CustomerRenamedList> for List {
    fn apply(&mut self, event: CustomerRenamedList) {
        self.name = event.new
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

impl DomainEvent<List> for ListEvent {
    fn apply(self, aggregate: &mut List) {
        match self {
            ListEvent::CustomerCreatedList(e) => aggregate.apply(e),
            ListEvent::CustomerRenamedList(e) => aggregate.apply(e),
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
        list.create(self)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CustomerRenameList {
    new: String,
}

impl Command<List, ListEvent> for CustomerRenameList {
    fn handle(self, list: &List) -> Result<Vec<ListEvent>, AggregateError> {
        list.rename(self)
    }
}

#[cfg(test)]
mod tests {
    use cqrs_es::test::TestFramework;

    use crate::domain::shopping;

    type ShoppingListSuite = TestFramework<shopping::list::List, shopping::list::ListEvent>;

    #[test]
    fn it_creates_a_list() {
        ShoppingListSuite::default()
            .given_no_previous_events()
            .when(shopping::list::CustomerCreateList {
                id: "list-uuid-1".to_string(),
                name: "List name".to_string(),
            })
            .then_expect_events(vec![shopping::list::ListEvent::CustomerCreatedList(
                shopping::list::CustomerCreatedList {
                    id: "list-uuid-1".to_string(),
                    name: "List name".to_string(),
                },
            )]);
    }

    #[test]
    fn it_renames_a_list() {
        ShoppingListSuite::default()
            .given(vec![shopping::list::ListEvent::CustomerCreatedList(
                shopping::list::CustomerCreatedList {
                    id: "list-uuid-1".to_string(),
                    name: "List name".to_string(),
                },
            )])
            .when(shopping::list::CustomerRenameList {
                new: "New list name".to_string(),
            })
            .then_expect_events(vec![shopping::list::ListEvent::CustomerRenamedList(
                shopping::list::CustomerRenamedList {
                    former: "List name".to_string(),
                    new: "New list name".to_string(),
                },
            )]);
    }

    #[test]
    fn it_does_not_rename_when_got_same_name() {
        ShoppingListSuite::default()
            .given(vec![shopping::list::ListEvent::CustomerCreatedList(
                shopping::list::CustomerCreatedList {
                    id: "list-uuid-1".to_string(),
                    name: "List name".to_string(),
                },
            )])
            .when(shopping::list::CustomerRenameList {
                new: "List name".to_string(),
            })
            .then_expect_events(vec![]);
    }
}
