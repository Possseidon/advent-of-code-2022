use lazy_regex::{regex, regex_captures};

pub fn part1(input: String) -> String {
    let (crate_stack, move_operations) = input.split_once("\n\n").unwrap();

    let mut crate_stack = CrateStack::parse(crate_stack);
    let move_operations = MoveOperations::parse(move_operations);

    for move_operation in move_operations.0 {
        crate_stack.apply_move_operation_9000(move_operation);
    }

    crate_stack.top_row()
}

pub fn part2(input: String) -> String {
    let (crate_stack, move_operations) = input.split_once("\n\n").unwrap();

    let mut crate_stack = CrateStack::parse(crate_stack);
    let move_operations = MoveOperations::parse(move_operations);

    for move_operation in move_operations.0 {
        crate_stack.apply_move_operation_9001(move_operation);
    }

    crate_stack.top_row()
}

#[derive(Debug)]
struct CrateStack(Vec<Vec<Crate>>);

impl CrateStack {
    fn parse(input: &str) -> Self {
        let mut result = Vec::<Vec<Crate>>::new();

        let rows = input
            .lines()
            .rev()
            .skip(1)
            .map(|line| line.chars().skip(1).step_by(4));

        for row in rows {
            for (i, crate_contents) in row.enumerate() {
                if crate_contents == ' ' {
                    continue;
                }

                if i >= result.len() {
                    result.resize_with(i + 1, Default::default)
                }

                result[i].push(Crate(crate_contents));
            }
        }

        Self(result)
    }

    fn apply_move_operation_9000(&mut self, move_operation: MoveOperation) {
        for _ in 0..move_operation.count {
            let c = self.0[move_operation.from].pop().unwrap();
            self.0[move_operation.to].push(c);
        }
    }

    fn apply_move_operation_9001(&mut self, move_operation: MoveOperation) {
        let from = &mut self.0[move_operation.from];
        let split_point = from.len() - move_operation.count;
        let c = from.split_off(split_point);
        self.0[move_operation.to].extend(c);
    }

    fn top_row(&self) -> String {
        self.0.iter().map(|stack| stack.last().unwrap().0).collect()
    }
}

#[derive(Debug)]
struct Crate(char);

#[derive(Debug)]
struct MoveOperations(Vec<MoveOperation>);
impl MoveOperations {
    fn parse(move_operations: &str) -> Self {
        Self(
            move_operations
                .lines()
                .map(|line| {
                    let (_, count, from, to) =
                        regex_captures!(r#"move (\d+) from (\d+) to (\d+)"#, line).unwrap();
                    MoveOperation {
                        count: count.parse().unwrap(),
                        from: from.parse::<usize>().unwrap() - 1,
                        to: to.parse::<usize>().unwrap() - 1,
                    }
                })
                .collect(),
        )
    }
}

#[derive(Debug)]
struct MoveOperation {
    count: usize,
    from: usize,
    to: usize,
}
