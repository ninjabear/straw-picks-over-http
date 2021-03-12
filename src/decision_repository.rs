use super::decision::*;

pub struct DecisionRepository {
    decisions: Vec<Decision>,
}

impl DecisionRepository {
    pub fn new() -> DecisionRepository {
        DecisionRepository { decisions: vec![] }
    }

    pub fn put(&mut self, decision: Decision) {
        self.decisions.push(decision);
        if self.decisions.len() > 100 {
            self.decisions.remove(0);
        }
    }

    pub fn all(&self) -> &Vec<Decision> {
        &self.decisions
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_empty_new() {
        let r = DecisionRepository::new();
        assert!(r.all().is_empty())
    }

    #[test]
    fn test_put_has_element() {
        let mut r = DecisionRepository::new();
        r.put(Decision::new("foo", &vec!["foo"]));
        assert_eq!(r.all().len(), 1);
        assert_eq!("foo", r.all()[0].pick);
    }

    #[test]
    fn test_no_more_than_n_records() {
        let mut r = DecisionRepository::new();
        r.put(Decision::new("first", &vec!["first"]));

        for _ in 1..100 {
            r.put(Decision::new("middle", &vec!["middle"]));
        }
        assert_eq!(r.all().len(), 100);

        r.put(Decision::new("last", &vec!["last"]));

        assert_eq!(r.all().len(), 100);
        assert_eq!(r.all()[99].pick, "last");
        assert_eq!(r.all()[0].pick, "middle");
    }
}
