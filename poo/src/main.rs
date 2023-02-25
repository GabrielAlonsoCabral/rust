fn main() {
    let mut averaged_collection: AveragedCollection = AveragedCollection::new();

    averaged_collection.show();
    averaged_collection.add(10);
    averaged_collection.show();
    averaged_collection.add(20);
    averaged_collection.show();
    averaged_collection.remove();
    averaged_collection.show();
}

#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn show(&self) {
        println!("{:?}", self)
    }
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result: Option<i32> = self.list.pop();

        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
