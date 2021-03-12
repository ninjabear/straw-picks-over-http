use super::decision::*;
use chrono::{DateTime, SecondsFormat, Utc};
use itertools::Itertools;

pub struct Ticket {
    pub decision: Decision,
    pub last_n_decisions: Vec<Decision>,
}

impl Ticket {
    pub fn new(decision: Decision, prev: Vec<Decision>) -> Ticket {
        Ticket {
            decision: decision,
            last_n_decisions: prev,
        }
    }
}

fn format_choices(c: &Vec<String>) -> String {
    c.iter().map(|s| format!("'{}'", s)).join(",")
}

pub fn write(t: &Ticket) -> String {
    format!(
        "{} has been selected. Choices were: {}.\n\n\
        Reference: {}\n\
        Time: {}\n\n\
        Recent selections:\n\n\
        {}",
        t.decision.pick,
        format_choices(&t.decision.choices),
        t.decision.reference.to_string(),
        format_datetime_string(&t.decision.issue_time),
        t.last_n_decisions
            .iter()
            .sorted_by(|a, b| Ord::cmp(&b.issue_time, &a.issue_time))
            .map(|d| format!(
                "\t{} was selected from [{}] at {} [{}]",
                d.pick,
                format_choices(&d.choices),
                format_datetime_string(&d.issue_time),
                d.reference
            ))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn format_datetime_string(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::SecondsFormat;

    fn sample_ticket() -> Ticket {
        let decision = Decision::new("Ed", &vec!["Ed"]);
        let prev = vec![
            Decision::new("Alvin", &vec!["Alvin"]),
            Decision::new("Simon", &vec!["Simon"]),
            Decision::new("Theo", &vec!["Theo"]),
        ];
        return Ticket::new(decision, prev);
    }

    fn format_dtime(dt: &DateTime<Utc>) -> String {
        dt.to_rfc3339_opts(SecondsFormat::Millis, true)
    }

    #[test]
    fn test_new_stores_elements() {
        let decision = Decision::new("foo", &vec!["foo"]);
        let decision_ref = decision.reference.clone();
        let old_decisions = vec![
            Decision::new("foo", &vec!["foo"]),
            Decision::new("bar", &vec!["bar"]),
            Decision::new("foo2", &vec!["foo2"]),
        ];
        let ticket = Ticket::new(decision, old_decisions);

        assert_eq!(decision_ref, ticket.decision.reference);
    }

    #[test]
    fn test_write_matches_format() {
        let actual = sample_ticket();
        let expected: String = format!(
            "{} has been selected. Choices were: {}.\n\n\
            Reference: {}\n\
            Time: {}\n\n\
            Recent selections:\n\n\
            {}",
            &actual.decision.pick,
            &actual
                .decision
                .choices
                .iter()
                .map(|c| format!("'{}'", c))
                .join(","),
            &actual.decision.reference.to_string(),
            format_dtime(&actual.decision.issue_time),
            &actual
                .last_n_decisions
                .iter()
                .sorted_by(|a, b| Ord::cmp(&b.issue_time, &a.issue_time))
                .map(|d| format!(
                    "\t{} was selected from [{}] at {} [{}]",
                    d.pick,
                    d.choices.iter().map(|c| format!("'{}'", c)).join(","),
                    format_dtime(&d.issue_time),
                    d.reference.to_string()
                ))
                .collect::<Vec<String>>()
                .join("\n")
        );
        assert_eq!(expected, write(&actual))
    }
}
