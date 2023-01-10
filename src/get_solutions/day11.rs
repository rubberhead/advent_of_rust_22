use super::AOCSolutions; 

type Item = usize; 

struct Monkey {
    items: Vec<Item>,
    inspection_count: usize, 

    /**
    Operation to be performed when the monkey inspects an item in its inventory.
     */
    op: Box<dyn Fn(Item) -> Item>, 

    /**
    Test to be performed when the monkey is to throw an item to another monkey. 

    Returns ident of target `Monkey` -- driver code is responsible for making sure that that `Monkey` 
    actually exists.
     */
    test: Box<dyn Fn(Item) -> usize>, 
}

impl Monkey {
    pub fn inspect_item(&mut self) -> Option<Item> {
        match self.items.pop() {
            Some(item) => {
                self.inspection_count += 1;
                Some((self.op)(item))
            }, 
            None => None, 
        }
    }

    pub fn find_item_target(&self, item: Item) -> usize {
        (self.test)(item)
    }

    pub fn receive_item(&mut self, item: Item) {
        self.items.push(item); 
    }

    pub fn minimize_inspection_op(&mut self) {
        self.op = Box::new(|item| item ); 
    }
}

fn play_round(mut monkeys: Vec<Monkey>, reaction_to_inspection: impl Fn(Item) -> Item) -> Vec<Monkey> {
    let mut sent_items_buffer: Vec<Vec<Item>> = vec![Vec::new(); monkeys.len()]; 

    // Iterate through all monkeys in sequence
    for (idx, monke) in monkeys.iter_mut().enumerate() {
        monke.items.append(&mut sent_items_buffer[idx]); 
        sent_items_buffer[idx].clear(); 
        while let Some(item) = monke.inspect_item() {
            let item = reaction_to_inspection(item); 
            let recv_idx = monke.find_item_target(item); 
            if recv_idx >= sent_items_buffer.len() { 
                panic!("[day9::play_round] Undefined monkey with index `{}` > `{}`", recv_idx, sent_items_buffer.len() - 1); 
            }
            sent_items_buffer[recv_idx].push(item); 
        }
    }

    // Finally, sent_items_buffer resulted from last monkey need to be applied to monkeys again
    for i in 0..monkeys.len() {
        monkeys[i].items.append(&mut sent_items_buffer[i]); 
    }

    return monkeys; 
}

fn parse_monkeys(input: &str) -> (Vec<Monkey>, usize) {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut prod_div = 1 as usize; 
    for monke_description in input.split("\n\n") { // Construct monkeys 
        let monke_description: Vec<&str> = monke_description.lines()
            .map(|s| match s.split_once(':') {
                Some((_, r)) => r.trim(), 
                None => panic!("[day9::parse_monkeys] Malformed input in description \"{}\"", s.trim()), 
            })
            .collect(); // 6 lines if well-formed
        if monke_description.len() != 6 {
            panic!("[day9::parse_monkeys] Malformed input in description: Malformed field count"); 
        }

        monkeys.push(Monkey {
            // self.items
            items: monke_description[1].split(", ")
                .map(|s| s.parse().expect(&format!("[day9::parse_monkeys] Malformed starting items: \"{}\"", monke_description[1])) )
                .collect(),

            // self.inspection_count 
            inspection_count: 0, 

            // self.op
            op: if let [amnt_str, op_str] = monke_description[2].split_whitespace().rev().take(2).collect::<Vec<&str>>()[..] {
                match amnt_str {
                    "old" => match op_str {
                        "*" => Box::new(|old| old * old), 
                        "+" => Box::new(|old| old + old), 
                        _   => panic!("[day9::parse_monkeys] Malformed operation: \"{}\"", monke_description[2]), 
                    }, 
                    _ => {
                        let amnt: usize = amnt_str.parse().expect(&format!("[Day9::get_star_1] Malformed operation: \"{}\"", monke_description[2]));
                        match op_str {
                            "*" => Box::new(move |old| old * amnt), 
                            "+" => Box::new(move |old| old + amnt), 
                            _   => panic!("[day9::parse_monkeys] Malformed operation: \"{}\"", monke_description[2]), 
                        }
                    }
                }
            } else {
                panic!("[day9::parse_monkeys] Malformed operation: \"{}\"", monke_description[2]); 
            }, 

            // self.test
            test: if let [divisor, true_idx, false_idx] = monke_description[3..].iter()
                .map(|s| {
                    s.split_whitespace()
                        .find_map(|w| if let Ok(n) = w.parse::<usize>() { Some(n) } else { None })
                        .expect(&format!("[day9::parse_monkeys] Malformed test: \n\"{}\n{}\n{}\"", monke_description[3], monke_description[4], monke_description[5]))
                })
                .collect::<Vec<_>>()[..] 
            {
                prod_div *= divisor; // prod_div is used to reduce common terms: 
                Box::new(move |item| if item % divisor == 0 { true_idx } else { false_idx } )
            } else {
                panic!("[day9::parse_monkeys] Malformed test: \n\"{}\n{}\n{}\"", monke_description[3], monke_description[4], monke_description[5]); 
            }, 
        });
    }

    return (monkeys, prod_div); 
}

pub struct Day11; 

impl AOCSolutions for Day11 {
    fn get_star_1(input: &str) -> Result<i64, ()> {
        let reaction = |item: Item| -> Item { item / 3 };
        let mut monkeys: Vec<Monkey> = parse_monkeys(input).0; 
        
        for _ in 1..=20 as usize {
            monkeys = play_round(monkeys, reaction); 
        }

        if monkeys.len() < 2 { return Ok(monkeys[0].inspection_count.try_into().unwrap()) }
        monkeys.sort_by(|a, b| a.inspection_count.partial_cmp(&b.inspection_count).unwrap().reverse() ); 
        return Ok((monkeys[0].inspection_count * monkeys[1].inspection_count).try_into().unwrap());
    }

    fn get_star_2(input: &str) -> Result<i64, ()> {
        let (mut monkeys, prod_div) = parse_monkeys(input); 

        for _ in 1..=10000 as usize {
            monkeys = play_round(monkeys, |item| { item % prod_div }); // Learned trick... I myself am not good at modular arithmetic
        }

        if monkeys.len() < 2 { return Ok(monkeys[0].inspection_count.try_into().unwrap()) }
        monkeys.sort_by(|a, b| a.inspection_count.partial_cmp(&b.inspection_count).unwrap().reverse() ); 
        return Ok((monkeys[0].inspection_count * monkeys[1].inspection_count).try_into().unwrap());
    }
}


#[cfg(test)]
mod tests {
    use super::Monkey;
    use super::Day11;
    use super::AOCSolutions; 

    const SAMPLE_INPUT: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"; 

    fn init_test_env() -> Vec<Monkey> {
        vec![
            Monkey {
                items: vec![79, 98], 
                op: Box::new(|old| old * 19), 
                test: Box::new(|item| if item % 23 == 0 { 2 } else { 3 }), 
                inspection_count: 0, 
            }, 
            Monkey {
                items: vec![54, 65, 75, 74], 
                op: Box::new(|old| old + 6), 
                test: Box::new(|item| if item % 19 == 0 { 2 } else { 0 }), 
                inspection_count: 0, 
            }, 
            Monkey {
                items: vec![79, 60, 97], 
                op: Box::new(|old| old * old), 
                test: Box::new(|item| if item % 13 == 0 { 1 } else { 3 }), 
                inspection_count: 0, 
            }, 
            Monkey {
                items: vec![74], 
                op: Box::new(|old| old + 3), 
                test: Box::new(|item| if item % 17 == 0 { 0 } else { 1 }), 
                inspection_count: 0, 
            }, 
        ]
    }

    #[test]
    fn test_play_round() {
        let mut monkeys = init_test_env(); 
        monkeys = super::play_round(monkeys, |item| item / 3 ); 
        assert!(([20, 23, 27, 26] as [usize; 4]).iter().all(|x| monkeys[0].items.contains(x) )); 
        assert_eq!(monkeys[0].items.len(), 4); 
        assert!(([2080, 25, 167, 207, 401, 1046] as [usize; 6]).iter().all(|x| monkeys[1].items.contains(x) )); 
        assert_eq!(monkeys[1].items.len(), 6); 
        assert!(monkeys[2].items.is_empty()); 
        assert!(monkeys[3].items.is_empty()); 

        for _ in 2..=20 {
            monkeys = super::play_round(monkeys, |item| item / 3); 
        }

        assert!(([10, 12, 14, 26, 34] as [usize; 5]).iter().all(|x| monkeys[0].items.contains(x) )); 
        assert_eq!(monkeys[0].items.len(), 5); 
        assert!(([245, 93, 53, 199, 115] as [usize; 5]).iter().all(|x| monkeys[1].items.contains(x) )); 
        assert_eq!(monkeys[1].items.len(), 5); 
        assert!(monkeys[2].items.is_empty()); 
        assert!(monkeys[3].items.is_empty()); 

        assert_eq!(monkeys.iter().map(|m| m.inspection_count ).collect::<Vec<usize>>(), vec![101, 95, 7, 105]); 
    }

    #[test]
    fn test_get_star_1() {
        assert_eq!(Day11::get_star_1(SAMPLE_INPUT).unwrap(), 10605); 
    }

    #[test]
    fn test_get_star_2() {
        assert_eq!(Day11::get_star_2(SAMPLE_INPUT).unwrap(), 2713310158); 
    }
}





