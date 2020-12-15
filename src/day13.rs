use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day13, part1)]
fn part1(data: &Vec<String>) -> i32 {
    let data = data.clone();

    let earliest = data[0].parse::<i32>().unwrap();
    let bus_ids: Vec<i32> = data[1]
        .split(',')
        .filter(|f| f != &"x")
        .map(|b| b.parse::<i32>().unwrap())
        .collect();

    let mut leave = earliest;
    let mut _bus_id = 0;

    loop {
        match bus_ids.iter().find(|a| leave % *a == 0) {
            Some(v) => {
                _bus_id = *v;
                break;
            }
            None => leave += 1,
        }
    }
    println!(
        "start:{:?} earliest:{:?} bus_id:{:?}",
        earliest, leave, _bus_id
    );
    (leave - earliest) * _bus_id
}

#[aoc(day13, part2)]
fn part2(data: &Vec<String>) -> i64 {
    let data = data.clone();
    let mut buses: Vec<(i64, i64)> = data[1]
        .split(',')
        .enumerate()
        .filter(|(_, f)| f != &"x")
        .map(|(i, b)| (i as i64, b.parse::<i64>().unwrap()))
        .collect();

    //Sorting to do the maximum first reduces our loops as we need to match smaller numbers.
    buses.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{:?}", buses);

    let mut time = 0;
    let mut step = 1;
    let mut count = 0;

    for (offset, bus) in buses {
        while (time + offset) % bus != 0 {
            time += step;
            count += 1;
        }
        //We can take larger steps each time
        println!("time:{:?} step:{:?} count:{:?}", time, step, count);
        step *= bus;
    }

    println!("Problem 2 - Lowest Time {:?} in {:?} loops", time, count);
    count
}
