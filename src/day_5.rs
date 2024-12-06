use std::collections::{HashMap, HashSet};

pub fn part_1(input: &str) -> String {
    let (page_ordering_rules, updates) = parse_input(input);

    let mut ans = 0;
    for line in updates.lines() {
        let pages = line
            .split(',')
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // Pages which need to be printed regardless of order
        let update_pages: HashSet<i32> = pages.iter().cloned().collect();

        let mut right_order: bool = true;
        let mut printed_pages = HashSet::new();
        for page in &pages {
            // Identify if the current page has any rules
            if let Some(rule_required_pages) = page_ordering_rules.get(&page) {
                // Identify which pages are required based on the pages to be printed
                let required_pages = update_pages
                    .intersection(rule_required_pages)
                    .cloned()
                    .collect::<HashSet<i32>>();
                // Only the subset from the rules which also need to be printed need to be checked against
                if !required_pages.is_subset(&printed_pages) {
                    right_order = false;
                    break;
                }
            }

            printed_pages.insert(*page);
        }

        if right_order {
            // Assume that there is always an odd number of pages for there to be a middle page
            ans += pages[(pages.len() - 1) / 2];
        }
    }

    ans.to_string()
}

// Some characteristics I observed by testing my input
// - There are always an odd number of pages for an update
// - No page is duplicated in an update
// - Each page in an update has a unique number of pages to be printed before it
// The last two properties imply that the correct order can be determined simply by the number of pages each page requires to be printed before it
// Incidentally this also applies for the ones already in the correct order
pub fn part_2(input: &str) -> String {
    let (page_ordering_rules, updates) = parse_input(input);

    let mut ans = 0;
    for line in updates.lines() {
        let pages = line
            .split(',')
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // Pages which need to be printed regardless of order
        let update_pages: HashSet<i32> = pages.iter().cloned().collect();

        let mut right_order: bool = true;
        let mut printed_pages = HashSet::new();

        let middle_page_index = (pages.len() - 1) / 2;

        // This will be discovered by iterating through the pages
        let mut correct_middle_page = None;

        for page in &pages {
            // Identify if the current page has any rules
            if let Some(rule_required_pages) = page_ordering_rules.get(&page) {
                // Identify which pages are required based on the pages to be printed
                let required_pages = update_pages
                    .intersection(rule_required_pages)
                    .cloned()
                    .collect::<HashSet<i32>>();
                if required_pages.len() == middle_page_index {
                    correct_middle_page = Some(*page);
                }
                if !required_pages.is_subset(&printed_pages) {
                    right_order = false;
                }
            }

            printed_pages.insert(*page);
        }

        if !right_order {
          // We are guaranteed to know the correct middle page based on the above observations
            ans += correct_middle_page.unwrap();
        }
    }

    ans.to_string()
}

// page_ordering_rules is a map describing the pages that must be printed before a given page
// The set of required pages includes pages which might not need to be printed
fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, String) {
    let normalized_input = input.replace("\r\n", "\n");

    let sections = normalized_input.split("\n\n").collect::<Vec<&str>>();

    let page_ordering_rules_section = sections[0];
    let updates_section = sections[1].to_string();

    let mut page_ordering_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for line in page_ordering_rules_section.lines() {
        let values = line.split_once('|').unwrap();
        let x = values.0.trim().parse::<i32>().unwrap();
        let y = values.1.trim().parse::<i32>().unwrap();

        if page_ordering_rules.contains_key(&y) {
            page_ordering_rules.get_mut(&y).unwrap().insert(x);
        } else {
            page_ordering_rules.insert(y, HashSet::from([x]));
        }
    }

    (page_ordering_rules, updates_section)
}
