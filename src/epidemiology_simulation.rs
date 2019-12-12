use crate::agent;
use crate::allocation_map;
use std::time::SystemTime;
use crate::constants;
use crate::geography::point;
use crate::geography::housing_area::HousingArea;
use crate::geography::work_area::WorkArea;
use crate::geography::point::Point;
use crate::csv_service;
use crate::geography::transport_area::TransportArea;

pub struct Epidemiology {
    pub agent_location_map: allocation_map::AgentLocationMap,
    pub housing_area: HousingArea,
    pub work_area: WorkArea,
    pub transport_area: TransportArea
}

impl Epidemiology {

    pub fn new(grid_size: i32, number_of_agents: i32) -> Epidemiology {
//        TODO: Change allocation strategy
        let x_offset_for_home = grid_size / constants::HOUSE_AREA_RELATED_SIZE;
        let x_offset_for_work = x_offset_for_home + constants::TRANSPORT_AREA_SIZE+1;

        let (housing_area, transport_area, work_area) = Epidemiology::define_geography(grid_size, x_offset_for_home, x_offset_for_work);
        let (home_locations, agent_list) = Epidemiology::generate_population(grid_size, number_of_agents, x_offset_for_home);

        let agent_location_map = allocation_map::AgentLocationMap::new(grid_size, &agent_list, &home_locations);
        Epidemiology{agent_location_map, housing_area, work_area, transport_area}
    }

//    TODO: Stop the simulation if the infection dies out
    pub fn run(&mut self, simulation_life_time:i32, vaccination_time:i32, vaccination_percentage:f64, output_file_name: &str) {
        let mut records: Vec<csv_service::Row> = Vec::new();
        println!("Tick 0");

        self.agent_location_map.goto(self.housing_area);
        for i in 1..simulation_life_time {
            self.agent_location_map.counts.increment_hour();
            let start_time = SystemTime::now();
            self.routine(i);

            if i == vaccination_time{
                self.agent_location_map.vaccinate(vaccination_percentage);
            }

            let end_time = SystemTime::now();
            records.push(self.agent_location_map.get_record());
            println!("Tick {}, Time taken {:?}", i, end_time.duration_since(start_time));
//            self.agent_location_map.print();
        }

        let result = csv_service::write(output_file_name, &records);
        match result{
            Ok(_) => {},
            Err(e) => {println!("Error occurred while writing data to csv {:?}", e)}
        }
    }

    fn define_geography(grid_size: i32, x_offset_for_home: i32, x_offset_for_work: i32) -> (HousingArea, TransportArea, WorkArea) {
        let housing_area: HousingArea = HousingArea::new(Point::new(0, 0), Point::new(x_offset_for_home, grid_size));
        let transport_area = TransportArea::new(Point::new(x_offset_for_home + 1, 0), Point::new(x_offset_for_work - 1, grid_size));
        let work_area = WorkArea::new(Point::new(x_offset_for_work, 0), Point::new(grid_size, grid_size));
        (housing_area, transport_area, work_area)
    }

    fn generate_population(grid_size: i32, number_of_agents: i32, x_offset_for_home: i32) -> (Vec<Point>, Vec<agent::Citizen>) {
        let home_location_boundary = Point::new(x_offset_for_home, grid_size);
        let home_locations = point::point_factory(Point::new(0, 0),
                                                  home_location_boundary, number_of_agents);
        let work_locations = home_locations.iter()
            .map(|x| *x + point::Point::new(x_offset_for_home, 0)).collect();
        let agent_list = agent::citizen_factory(&home_locations, &work_locations);
        (home_locations, agent_list)
    }

    fn routine(&mut self, i: i32) {
        match i % constants::NUMBER_OF_HOURS {
            constants::ROUTINE_START_TIME => {
                self.agent_location_map.update_infections();
                self.agent_location_map.update_infection_day();
                self.agent_location_map.quarantine();
            },
            constants::ROUTINE_TRAVEL_START_TIME => self.agent_location_map.goto(self.transport_area),
            constants::ROUTINE_WORK_TIME => self.agent_location_map.goto(self.work_area),
            constants::ROUTINE_TRAVEL_END_TIME => self.agent_location_map.goto(self.transport_area),
            constants::ROUTINE_WORK_END_TIME => self.agent_location_map.goto(self.housing_area),
            constants::ROUTINE_END_TIME => { self.agent_location_map.deceased(); },
            _ => self.agent_location_map.move_agents()
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

//    #[test]
//    fn init() {
//        let epidemiology:Epidemiology = Epidemiology::new(3, 3);
//
//        assert_eq!(epidemiology.agent_list.len(), 3);
//    }
}