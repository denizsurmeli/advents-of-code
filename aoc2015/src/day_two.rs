struct Object {
    x: u32,
    y: u32,
    z: u32,
}

impl Object {
    pub fn new(dimensions: &str) -> Object {
        let dimensions: Vec<u32> = dimensions
            .split("x")
            .map(|e| e.parse::<u32>().unwrap())
            .collect();
        Object {
            x: dimensions[0],
            y: dimensions[1],
            z: dimensions[2],
        }
    }

    pub fn surface_area(&self) -> u32 {
        let surfaces = vec![
            2 * self.x * self.y,
            2 * self.x * self.z,
            2 * self.z * self.y,
        ];
        let minimum = *(&surfaces).into_iter().min().unwrap() as u32 / 2 as u32;
        surfaces.into_iter().sum::<u32>() + minimum
    }

    pub fn ribbon(&self) -> u32 {
        let mut temp = vec![self.x, self.y, self.z];
        temp.sort();
        2 * (temp[0] + temp[1]) + (temp[0] * temp[1] * temp[2])
    }
}

pub fn wrapping_paper(path: &str) -> u32 {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|e| Object::new(e).surface_area())
        .sum()
}

pub fn ribbon_needed(path: &str) -> u32 {
    std::fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|e| Object::new(e).ribbon())
        .sum()
}
