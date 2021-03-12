use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Decision {
    pub reference: Uuid,
    pub issue_time: DateTime<Utc>,
    pub pick: String,
    pub choices: Vec<String>,
}

impl Decision {
    pub fn new(pick: &str, choices: &Vec<&str>) -> Self {
        let reference = Uuid::new_v4();

        Decision {
            reference: reference,
            issue_time: Utc::now(),
            pick: pick.to_string(),
            choices: choices.iter().map(|s| s.to_string()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::*;

    use chrono::Duration;

    #[test]
    fn test_make_decision_pick_uses_str() {
        let d = Decision::new("foo", &vec!["foo"]);
        assert_eq!("foo", d.pick)
    }

    #[test]
    fn test_make_decision_ticket_refs_differ() {
        let decision_count = 100;
        let unique_ticket_ref_count = (1..decision_count + 1)
            .map(|_| Decision::new("foo", &vec!["foo"]).reference.to_string())
            .collect::<HashSet<String>>()
            .len();
        assert_eq!(decision_count, unique_ticket_ref_count);
    }

    #[test]
    fn test_time_like_now() {
        let actual = &Decision::new("foo", &vec!["foo"]).issue_time;
        let after = Utc::now();

        if actual > &after {
            panic!("generated decision time in the future");
        }

        if actual < &(after - Duration::seconds(5)) {
            panic!("generated decision time is more than 5 seconds ago")
        }
    }
}
