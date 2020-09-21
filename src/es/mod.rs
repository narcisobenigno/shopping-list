use std::time::Instant;

trait Aggregate {
    fn apply(e: Event<Aggregate>);
}

struct Event<A>
where
    A: Aggregate, {}

struct EventEntry<A>
where
    A: Aggregate,
{
    timestamp: Instant,
    position: u128,
    version: u32,
    aggregate_id: &str,
    payload: Event<A>,
}

trait AggregateLoader {
    fn load<A>(uuid: String) -> A
    where
        A: Aggregate;
}
