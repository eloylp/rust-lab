//! Welcome to the state pattern module !
//!
//! Here we will go thorough some common rust examples
//! for creating awesome state machines.
//!
//! In the following example, we will see a state machine
//! implemented with generics and a marker type. This
//! kind of implementation allow us to avoid the incorrect
//! use of the API by introducing compile time constraints,
//! saving us many validations.
//!
//! We will implement the very basic steps of the flight plan
//! of an airplane !
use std::marker::PhantomData;

// Plane states
struct Parked;
struct Runway;
struct Takeoff;
struct Cruise;
struct Approach;

// As default, the plane will be in the Parked state.
struct Plane<State = Parked> {
    // Names of the passengers
    passengers: Vec<String>,
    gear_position: GearPosition,
    // See here the marker type. Its a zero sized type,
    // so it does not have memory cost.
    state: PhantomData<State>,
    // Airspeed in knots
    air_speed: u16,
    // Degrees
    pitch: i16,
    // Degrees
    heading: u16,
}

impl Plane<Parked> {
    pub fn new(heading: u16) -> Self {
        Plane {
            passengers: Vec::new(),
            gear_position: GearPosition::DOWN,
            state: PhantomData::<Parked>,
            air_speed: 0,
            pitch: 0,
            heading: heading,
        }
    }

    pub fn runway(&self) -> Plane<Runway> {
        Plane {
            passengers: self.passengers.to_owned(),
            gear_position: self.gear_position,
            state: PhantomData::<Runway>,
            air_speed: self.air_speed,
            pitch: self.pitch,
            heading: self.heading,
        }
    }

    pub fn onboard_passenger(&mut self, name: String) {
        println!("Onboarding {}", name);
        self.passengers.push(name);
    }

    fn offboard_passengers(&mut self) {
        while let Some(passenger) = self.passengers.pop() {
            println!("Offboarding {}", passenger)
        }
    }
}

impl Plane<Runway> {
    pub fn takeoff(&self) -> Plane<Takeoff> {
        Plane {
            passengers: self.passengers.to_owned(),
            gear_position: self.gear_position,
            state: PhantomData::<Takeoff>,
            air_speed: self.air_speed,
            pitch: self.pitch,
            heading: self.heading,
        }
    }

    pub fn park(&self) -> Plane<Parked> {
        Plane {
            passengers: self.passengers.to_owned(),
            gear_position: self.gear_position,
            state: PhantomData::<Parked>,
            air_speed: self.air_speed,
            pitch: self.pitch,
            heading: self.heading,
        }
    }

    pub fn increase_speed(&mut self, q: u16) {
        self.air_speed = self.air_speed.checked_add(q).unwrap();
    }

    pub fn decrease_speed(&mut self, q: u16) {
        self.air_speed = self.air_speed.checked_sub(q).unwrap();
    }

    fn increase_pitch(&mut self, q: i16) {
        self.pitch = self.pitch.checked_add(q).unwrap();
    }
}

impl Plane<Takeoff> {
    pub fn cruise(&self) -> Plane<Cruise> {
        Plane {
            passengers: self.passengers.to_owned(),
            gear_position: self.gear_position,
            state: PhantomData::<Cruise>,
            air_speed: self.air_speed,
            pitch: self.pitch,
            heading: self.heading,
        }
    }

    pub fn gear_up(&mut self) {
        self.gear_position = GearPosition::UP
    }
}

impl Plane<Cruise> {
    pub fn approach(&self) -> Plane<Approach> {
        Plane {
            passengers: self.passengers.to_owned(),
            gear_position: self.gear_position,
            state: PhantomData::<Approach>,
            air_speed: self.air_speed,
            pitch: self.pitch,
            heading: self.heading,
        }
    }
}

impl Maneuver for Plane<Cruise> {
    fn increase_pitch(&mut self, q: i16) {
        self.pitch = self.pitch.checked_add(q).unwrap();
    }

    fn decrease_pitch(&mut self, q: i16) {
        self.pitch = self.pitch.checked_sub(q).unwrap();
    }

    fn increase_speed(&mut self, q: u16) {
        self.air_speed = self.air_speed.checked_add(q).unwrap();
    }

    fn decrease_speed(&mut self, q: u16) {
        self.air_speed = self.air_speed.checked_sub(q).unwrap();
    }

    fn head_right(&mut self, q: u16) {
        self.heading = self.heading.checked_add(q).unwrap();
    }

    fn head_left(&mut self, q: u16) {
        self.heading = self.heading.checked_sub(q).unwrap();
    }
}

impl Plane<Approach> {
    pub fn land(&self) -> Plane<Runway> {
        Plane {
            passengers: self.passengers.to_owned(),
            gear_position: self.gear_position,
            state: PhantomData::<Runway>,
            air_speed: self.air_speed,
            pitch: self.pitch,
            heading: self.heading,
        }
    }

    pub fn gear_down(&mut self) {
        self.gear_position = GearPosition::DOWN
    }
}

impl Maneuver for Plane<Approach> {
    fn increase_pitch(&mut self, q: i16) {
        self.pitch = self.pitch.checked_add(q).unwrap();
    }

    fn decrease_pitch(&mut self, q: i16) {
        self.pitch = self.pitch.checked_sub(q).unwrap();
    }

    fn increase_speed(&mut self, q: u16) {
        self.air_speed = self.air_speed.checked_add(q).unwrap();
    }

    fn decrease_speed(&mut self, q: u16) {
        self.air_speed = self.air_speed.checked_sub(q).unwrap();
    }

    fn head_right(&mut self, q: u16) {
        self.heading = self.heading.checked_add(q).unwrap();
    }

    fn head_left(&mut self, q: u16) {
        self.heading = self.heading.checked_sub(q).unwrap();
    }
}

#[derive(Clone, Copy)]
enum GearPosition {
    UP,
    DOWN,
}

trait Maneuver {
    fn increase_pitch(&mut self, q: i16);
    fn decrease_pitch(&mut self, q: i16);
    fn increase_speed(&mut self, q: u16);
    fn decrease_speed(&mut self, q: u16);
    fn head_right(&mut self, q: u16);
    fn head_left(&mut self, q: u16);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_flow() {
        let mut plane = Plane::new(120);
        plane.onboard_passenger(String::from("Alice"));
        plane.onboard_passenger(String::from("Bob"));

        let mut plane_in_runway = plane.runway();
        plane_in_runway.increase_speed(100);
        plane_in_runway.increase_speed(80);
        plane_in_runway.increase_pitch(20);

        let mut plane_in_takeoff = plane_in_runway.takeoff();
        plane_in_takeoff.gear_up();

        let mut plane_in_cruise = plane_in_takeoff.cruise();
        plane_in_cruise.increase_speed(300);
        plane_in_cruise.decrease_pitch(20);
        plane_in_cruise.head_left(20);
        plane_in_cruise.increase_pitch(10);

        let mut plane_in_approach = plane_in_cruise.approach();
        plane_in_approach.decrease_pitch(10);
        plane_in_approach.decrease_speed(200);
        plane_in_approach.head_right(80);
        plane_in_approach.decrease_speed(100);
        plane_in_approach.gear_down();

        let mut plane_in_runway = plane_in_approach.land();
        plane_in_runway.decrease_speed(180);

        let mut plane_in_park = plane_in_runway.park();
        plane_in_park.offboard_passengers();
    }
}
