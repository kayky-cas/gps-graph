use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Coord(usize, usize);

#[derive(Clone)]
struct Link {
    distance: f32,
    end: Coord,
}

impl Link {
    fn new(distance: f32, end: Coord) -> Self {
        Self { distance, end }
    }
}

struct Star {
    name: String,
    coord: Coord,
    neighbors: Vec<Link>,
}

impl Star {
    fn new(name: &str, x: usize, y: usize) -> Star {
        Star {
            name: name.to_string(),
            coord: Coord(x, y),
            neighbors: Vec::new(),
        }
    }
}

type StarRef = Rc<RefCell<Star>>;

struct Space {
    stars: HashMap<Coord, StarRef>,
}

impl Space {
    fn new() -> Self {
        Self {
            stars: HashMap::new(),
        }
    }

    fn add_star(&mut self, star: Star) -> Result<(), ()> {
        if !self.stars.contains_key(&star.coord.clone()) {
            self.stars
                .insert(star.coord.clone(), Rc::new(RefCell::new(star)));
            Ok(())
        } else {
            Err(())
        }
    }

    fn create_link(&mut self, start: Coord, end: Coord, distance: f32) -> Result<(), ()> {
        let start = self.stars.get(&start);
        let end = self.stars.get(&end);

        if let (Some(star), Some(end)) = (start, end) {
            star.borrow_mut()
                .neighbors
                .push(Link::new(distance, end.borrow().coord.clone()));
            Ok(())
        } else {
            Err(())
        }
    }

    fn distance(&mut self, start: Coord, end: Coord) -> Result<f32, ()> {
        let start = self.stars.get(&start);
        let end = self.stars.get(&end);

        match (start, end) {
            (Some(start), Some(end)) => {
                let d = self.distance_star(start.clone(), end.clone());

                if d <= 0.0 {
                    Err(())
                } else {
                    Ok(d)
                }
            }
            _ => Err(()),
        }
    }

    fn distance_star(&mut self, start: StarRef, end: StarRef) -> f32 {
        if start.borrow().coord == end.borrow().coord {
            return 0.0;
        }

        start
            .borrow()
            .neighbors
            .iter()
            .map(|l| {
                let link = self
                    .stars
                    .get_mut(&l.end)
                    .expect("It's suppose that every child was check")
                    .clone();

                l.distance + self.distance_star(link, end.clone())
            })
            .min_by(|curr, oth| curr.partial_cmp(oth).unwrap())
            .unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Coord, Space, Star};

    #[test]
    fn insert_star() {
        let mut space = Space::new();
        let star = Star::new("Star", 0, 0);
        assert!(space.add_star(star).is_ok())
    }

    #[test]
    fn insert_star_where_exists() {
        let mut space = Space::new();

        let star = Star::new("Star", 0, 0);
        let star1 = Star::new("Star", 0, 0);

        assert!(space.add_star(star).is_ok());
        assert!(space.add_star(star1).is_err());
    }

    #[test]
    fn create_link_ok() {
        let mut space = Space::new();

        let coord = Coord(0, 0);
        let coord1 = Coord(1, 0);

        let star = Star::new("Star", coord.0, coord.1);
        let star1 = Star::new("Star", coord1.0, coord1.1);

        assert!(space.add_star(star).is_ok());
        assert!(space.add_star(star1).is_ok());

        assert!(space
            .create_link(coord.clone(), coord1.clone(), 10.0)
            .is_ok());
    }

    #[test]
    fn create_link_err_where_both_stars_not_exisits() {
        let mut space = Space::new();

        let coord = Coord(0, 0);
        let coord1 = Coord(1, 0);

        assert!(space
            .create_link(coord.clone(), coord1.clone(), 10.0)
            .is_err());
    }

    #[test]
    fn create_link_where_start_star_not_exisit() {
        let mut space = Space::new();

        let coord = Coord(0, 0);
        let coord1 = Coord(1, 0);

        let star1 = Star::new("Star", coord1.0, coord1.1);

        assert!(space.add_star(star1).is_ok());

        assert!(space
            .create_link(coord.clone(), coord1.clone(), 10.0)
            .is_err());
    }

    #[test]
    fn create_link_where_end_star_not_exisit() {
        let mut space = Space::new();

        let coord = Coord(0, 0);
        let coord1 = Coord(1, 0);

        let star1 = Star::new("Star", coord1.0, coord1.1);

        assert!(space.add_star(star1).is_ok());

        assert!(space
            .create_link(coord1.clone(), coord.clone(), 10.0)
            .is_err());
    }

    #[test]
    fn distance_not_exists() {
        let mut space = Space::new();

        let coord = Coord(0, 0);
        let coord1 = Coord(1, 0);

        let star = Star::new("Star", coord.0, coord.1);
        let star1 = Star::new("Star", coord1.0, coord1.1);

        assert!(space.add_star(star).is_ok());
        assert!(space.add_star(star1).is_ok());

        assert!(space.distance(coord.clone(), coord1.clone()).is_err());
    }

    #[test]
    fn distance_direct_is_10() {
        let mut space = Space::new();

        let coord = Coord(0, 0);
        let coord1 = Coord(1, 0);

        let star = Star::new("Star", coord.0, coord.1);
        let star1 = Star::new("Star", coord1.0, coord1.1);

        assert!(space.add_star(star).is_ok());
        assert!(space.add_star(star1).is_ok());

        assert!(space
            .create_link(coord.clone(), coord1.clone(), 10.0)
            .is_ok());

        assert_eq!(
            space
                .distance(coord.clone(), coord1.clone())
                .unwrap_or_else(|_| {
                    panic!(
                        "Should exists an distace between {:?} and {:?}",
                        coord, coord1
                    )
                }),
            10.0
        );
    }

    #[test]
    fn distance_not_direct_with_two_routes_is_10() {
        let mut space = Space::new();

        let coord = Coord(0, 0);
        let coord1 = Coord(1, 0);
        let coord2 = Coord(2, 0);
        let coord3 = Coord(3, 0);

        let star = Star::new("Star", coord.0, coord.1);
        let star1 = Star::new("Star", coord1.0, coord1.1);
        let star2 = Star::new("Star", coord2.0, coord2.1);
        let star3 = Star::new("Star", coord3.0, coord3.1);

        assert!(space.add_star(star).is_ok());
        assert!(space.add_star(star1).is_ok());
        assert!(space.add_star(star2).is_ok());
        assert!(space.add_star(star3).is_ok());

        assert!(space
            .create_link(coord.clone(), coord1.clone(), 5.0)
            .is_ok());

        assert!(space
            .create_link(coord.clone(), coord3.clone(), 10.0)
            .is_ok());

        assert!(space
            .create_link(coord1.clone(), coord2.clone(), 5.0)
            .is_ok());

        assert!(space
            .create_link(coord3.clone(), coord2.clone(), 5.0)
            .is_ok());

        assert_eq!(
            space
                .distance(coord.clone(), coord2.clone())
                .unwrap_or_else(|_| {
                    panic!(
                        "Should exists an distace between {:?} and {:?}",
                        coord, coord1
                    )
                }),
            10.0
        );
    }
}
