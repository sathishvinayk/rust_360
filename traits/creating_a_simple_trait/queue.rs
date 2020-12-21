use std::collections::VecDeque;

pub struct API {
    pub reqs: VecDeque<u32>
}

pub trait Queue {
    fn single_request(&mut self, req: u32);
    fn multiple_request(&mut self, reqs: &Vec<u32>);
    fn remove_request(&mut self) -> Option<u32>;
}

impl Queue for API {
    fn single_request(&mut self, req:u32) {
        self.reqs.push_back(req);
    }
    fn multiple_request(&mut self, reqs: &Vec<u32>){
        for req in reqs {
            self.reqs.push_back(*req);
        }
    }
    fn remove_request(&mut self) -> Option<u32> {
        self.reqs.pop_front()
    }
}