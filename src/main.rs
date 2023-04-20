use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, PartialEq, Eq, Hash)]
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
    pos: Coord,
    neighs: Vec<Link>,
}

impl Star {
    fn new(name: &str, x: usize, y: usize) -> Star {
        Star {
            name: name.to_string(),
            pos: Coord(x, y),
            neighs: Vec::new(),
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
        if self.stars.contains_key(&star.pos.clone()) {
            self.stars
                .insert(star.pos.clone(), Rc::new(RefCell::new(star)));
            Ok(())
        } else {
            Err(())
        }
    }

    fn create_link(&mut self, start: Coord, end: Coord, distance: f32) -> Result<(), ()> {
        if let Some(star) = self.stars.get(&start) {
            star.borrow_mut().neighs.push(Link::new(distance, end));
            Ok(())
        } else {
            Err(())
        }
    }

    fn distance(&mut self, start: Coord, end: Coord) -> Result<f32, ()> {
        let start = self.stars.get(&start);
        let end = self.stars.get(&end);

        match (start, end) {
            (Some(start), Some(end)) => Ok(self.distance_star(start.clone(), end.clone())),
            _ => Err(()),
        }
    }

    fn distance_star(&mut self, start: StarRef, end: StarRef) -> f32 {
        if start.borrow().pos == end.borrow().pos {
            return 0.0;
        }

        start
            .borrow()
            .neighs
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

fn main() {
    println!("Hello, world!");
}
