use crate::geography::Point;
use fxhash::FxHashMap;
use crate::events::{Listener, Counts};
use std::any::Any;

pub struct Hotspot {
    disease_hotspot_tracker: FxHashMap<Point, i32>
}

impl Hotspot {
    pub fn new() -> Hotspot {
        let disease_hotspot_tracker = FxHashMap::default();
        Hotspot{disease_hotspot_tracker}
    }
}

impl Listener for Hotspot {
    fn counts_updated(&mut self, _counts: Counts) {
    }

    fn simulation_ended(&mut self) {
    }

    fn citizen_got_infected(&mut self, cell: &Point) {
        let counter = self.disease_hotspot_tracker.entry(*cell).or_insert(0);
        *counter += 1;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests{
    use crate::disease_tracker::Hotspot;
    use fxhash::FxHashMap;
    use crate::geography::Point;
    use crate::events::Listener;

    #[test]
    fn should_initialize(){
        let tracker = Hotspot::new();
        assert_eq!(tracker.disease_hotspot_tracker.len(), 0);
    }

    #[test]
    fn should_add_new_entry(){
        let mut tracker = Hotspot::new();
        let current_point = Point::new(0, 1);

        tracker.citizen_got_infected(&current_point);

        assert_eq!(*tracker.disease_hotspot_tracker.get(&current_point).unwrap(), 1);
    }

    #[test]
    fn should_update_tracker(){
        let mut tracker = Hotspot::new();
        let current_point = Point::new(0, 1);

        tracker.citizen_got_infected(&current_point);
        tracker.citizen_got_infected(&current_point);

        assert_eq!(*tracker.disease_hotspot_tracker.get(&current_point).unwrap(), 2);
    }
}
