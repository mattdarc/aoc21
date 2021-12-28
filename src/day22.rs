use std::cmp::{max, min};
use std::ops::RangeInclusive;

struct ReactorCore {
    cubes: RegionTrie,
}

/// Struct to model a region. Regions alternate on-off. i.e. the root regions will all be on, their
/// children will be off, grandchildren on, etc.
#[derive(Clone)]
struct Region {
    on: bool,
    xr: RangeInclusive<i64>,
    yr: RangeInclusive<i64>,
    zr: RangeInclusive<i64>,
    sub_regions: Vec<Region>,
}

impl std::cmp::PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.xr.start() == other.xr.start()
            && self.yr.start() == other.yr.start()
            && self.zr.start() == other.zr.start()
            && self.xr.end() == other.xr.end()
            && self.yr.end() == other.yr.end()
            && self.zr.end() == other.zr.end()
    }
}

#[track_caller]
fn assert_disjoint(regions: &[Region]) {
    let mut found_overlap = false;
    for a in 0..regions.len() {
        for b in 0..a {
            if regions[a].intersects(&regions[b]) || regions[b].intersects(&regions[a]) {
                println!("Overlapping regions:\n{:?}, {:?}", regions[a], regions[b]);
                found_overlap = true;
            }
        }
    }

    assert!(!found_overlap);
}

impl std::fmt::Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.dbg_string())
    }
}

impl Region {
    fn world() -> RangeInclusive<i64> {
        i64::MIN..=i64::MAX
    }

    fn is_world(&self) -> bool {
        self.xr == Region::world() && self.yr == Region::world() && self.zr == Region::world()
    }

    fn from_command(command: &Command) -> Self {
        Region {
            on: command.turn_on,
            xr: command.xr.0..=command.xr.1,
            yr: command.yr.0..=command.yr.1,
            zr: command.zr.0..=command.zr.1,
            sub_regions: Vec::new(),
        }
    }

    fn new(
        xr: RangeInclusive<i64>,
        yr: RangeInclusive<i64>,
        zr: RangeInclusive<i64>,
        on: bool,
    ) -> Self {
        Region {
            on,
            xr,
            yr,
            zr,
            sub_regions: Vec::new(),
        }
    }

    fn split(&self, other: &Region) -> (Vec<Region>, Vec<Region>) {
        let find_subregions = |a: &RangeInclusive<i64>, b: &RangeInclusive<i64>| {
            let before = min(*a.start(), *b.start())..=max(*a.start(), *b.start()) - 1;
            let overlap = max(*a.start(), *b.start())..=min(*a.end(), *b.end());
            let after = 1 + min(*a.end(), *b.end())..=max(*a.end(), *b.end());
            vec![before, overlap, after]
        };

        let xr_regions = find_subregions(&self.xr, &other.xr);
        let yr_regions = find_subregions(&self.yr, &other.yr);
        let zr_regions = find_subregions(&self.zr, &other.zr);

        let mut other_regions = Vec::new();
        let mut self_regions = Vec::new();
        for xr in &xr_regions {
            for yr in &yr_regions {
                for zr in &zr_regions {
                    let mut new_region = Region::new(xr.clone(), yr.clone(), zr.clone(), false);
                    if other_regions.contains(&new_region)
                        || self_regions.contains(&new_region)
                        || new_region.is_empty()
                    {
                        continue;
                    }

                    if other.contains(&new_region) {
                        // Sub-Region is in the newly added one, set it to the same state
                        new_region.on = other.on;
                        other_regions.push(new_region);
                    } else if self.contains(&new_region) {
                        // Sub-Region is in the old region, same state as old
                        new_region.on = self.on;
                        self_regions.push(new_region);
                    }
                }
            }
        }
        assert!(
            self.volume() == self_regions.iter().map(|r| r.volume()).sum()
                || other.volume() == other_regions.iter().map(|r| r.volume()).sum()
        );

        (self_regions, other_regions)
    }

    fn is_empty(&self) -> bool {
        self.xr.is_empty() || self.yr.is_empty() || self.zr.is_empty()
    }

    /// Returns true if other is completely contained within self
    /// x0                x1
    /// +------------------+ y1
    /// |       self       |
    /// |                  |
    /// |   a0       a1    |
    /// |   +---------+ b1 |
    /// |   |         |    |
    /// |   |  other  |    |
    /// |   |         |    |
    /// |   +---------+ b0 |
    /// |                  |
    /// +------------------+ y0
    fn contains(&self, other: &Region) -> bool {
        other.xr.start() >= self.xr.start()
            && other.xr.end() <= self.xr.end()
            && other.yr.start() >= self.yr.start()
            && other.yr.end() <= self.yr.end()
            && other.zr.start() >= self.zr.start()
            && other.zr.end() <= self.zr.end()
    }

    /// Returns true if self splits other into overlapping and non-overlapping regions:
    /// x0        x1
    /// +----------+ y1
    /// |   self   |
    /// |          |
    /// |     a0   |   a1
    /// |     +----:----+ b1
    /// |     |    :    |
    /// |     |  other  |
    /// |     |    :    |
    /// |     +----:----+ b0
    /// |          |
    /// +----------+ y0
    ///
    /// x0        x1
    /// +----------+ y1
    /// |   self   |
    /// |          |
    /// |          |
    /// |     a0   |   a1
    /// |     + - -+----+ b1
    /// |     :    :    |
    /// +-----+- - + y0 |
    ///       |  other  |
    ///       |         |
    ///       +---------+ b0
    fn intersects(&self, other: &Region) -> bool {
        self.xr.start() <= other.xr.end()
            && self.xr.end() >= other.xr.start()
            && self.yr.start() <= other.yr.end()
            && self.yr.end() >= other.yr.start()
            && self.zr.start() <= other.zr.end()
            && self.zr.end() >= other.zr.start()
    }

    fn add_region(&mut self, other: Region) {
        // Find the sub regions that contain this region (at least partially). Split them up, and
        // add them back, then repeat the process with the remaining regions
        let mut regions = vec![other];
        while let Some(new_region) = regions.pop() {
            // Remove any sub-regions completely contained by this one. They are now the value of this
            // new region
            self.sub_regions.retain(|r| !new_region.contains(r));

            let intersected_region = self
                .sub_regions
                .iter()
                .enumerate()
                .find(|(_, r)| r.intersects(&new_region));

            if let Some((i, intersected_region)) = intersected_region {
                // Split the other region into sub-regions to be added, and try to add them
                let (mut self_regions, mut other_regions) = intersected_region.split(&new_region);
                regions.append(&mut other_regions);
                self.sub_regions.append(&mut self_regions);

                // Erase the old element from the array of sub regions
                self.sub_regions.remove(i);
            } else if new_region.on != self.on {
                // Simple case, no intersections
                self.sub_regions.push(new_region);
            }
        }
    }

    fn self_volume(&self) -> i64 {
        ((1 + self.xr.end() - self.xr.start())
            * (1 + self.yr.end() - self.yr.start())
            * (1 + self.zr.end() - self.zr.start())) as i64
    }

    fn volume(&self) -> i64 {
        if self.is_world() {
            return 0;
        }

        let self_volume = self.self_volume();
        let child_volume = self.sub_regions.iter().map(|r| r.volume()).sum::<i64>();

        self_volume - child_volume
    }

    fn dbg_string(&self) -> String {
        let state = if self.on { "on" } else { "off" };
        let child_volume = self.sub_regions.iter().map(|r| r.volume()).sum::<i64>();
        let s = format!(
            "{} ({:?}, {:?}, {:?}) -- {} - {}\n",
            state,
            self.xr,
            self.yr,
            self.zr,
            self.self_volume(),
            child_volume,
        );
        self.sub_regions
            .iter()
            .flat_map(|r| {
                r.dbg_string()
                    .lines()
                    .map(|l| "  ".to_owned() + &l)
                    .intersperse("\n".to_owned())
                    .collect::<Vec<_>>()
            })
            .fold(s, |s, sr| s + &sr)
    }
}

struct RegionTrie {
    root: Region,
}

impl RegionTrie {
    fn new() -> Self {
        RegionTrie {
            root: Region::new(Region::world(), Region::world(), Region::world(), false),
        }
    }

    fn add_region(&mut self, new_region: Region) {
        self.root.add_region(new_region);
        assert_disjoint(self.regions());
    }

    fn count_on(&self) -> i64 {
        self.regions().iter().map(|r| r.volume()).sum()
    }

    fn regions(&self) -> &[Region] {
        &self.root.sub_regions
    }
}

impl ReactorCore {
    fn new() -> Self {
        ReactorCore {
            cubes: RegionTrie::new(),
        }
    }

    fn execute_command(&mut self, command: &Command) {
        self.cubes.add_region(Region::from_command(command));
    }

    fn count_on(&self) -> i64 {
        self.cubes.count_on()
    }
}

#[derive(Debug, Clone)]
struct Command {
    xr: (i64, i64),
    yr: (i64, i64),
    zr: (i64, i64),
    turn_on: bool,
}

const CLAMP: i64 = 50;
fn clamp_50(r: (i64, i64)) -> (i64, i64) {
    (
        r.0.max(-1 * CLAMP).min(CLAMP),
        r.1.max(-1 * CLAMP).min(CLAMP),
    )
}

impl Command {
    fn restrict(&self) -> Self {
        Command {
            xr: clamp_50(self.xr),
            yr: clamp_50(self.yr),
            zr: clamp_50(self.zr),
            turn_on: self.turn_on,
        }
    }

    fn inside_init(&self) -> bool {
        let inside = |r: (i64, i64)| (r.0 >= -50 && r.0 <= 50) || (r.1 >= -50 && r.1 <= 50);
        inside(self.xr) && inside(self.yr) && inside(self.zr)
    }
}

#[aoc_generator(day22)]
fn parse_commands(input: &str) -> Vec<Command> {
    let range_re = regex::Regex::new(r"\w=(-?\d+)..(-?\d+)").unwrap();

    let mut commands = Vec::new();
    for line in input.lines().filter(|l| !l.is_empty()) {
        let (action_str, cubes) = line.split_once(' ').unwrap();
        let action = match action_str {
            "on" => true,
            "off" => false,
            _ => panic!("Unrecognized action!"),
        };

        let ranges = cubes
            .split(',')
            .map(|range| {
                let captures = range_re.captures(range).unwrap();
                let begin = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let end = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
                (begin, end)
            })
            .collect::<Vec<_>>();
        assert_eq!(ranges.len(), 3);

        commands.push(Command {
            xr: ranges[0],
            yr: ranges[1],
            zr: ranges[2],
            turn_on: action,
        });
    }

    commands
}

#[aoc(day22, part1)]
fn part1(commands: &[Command]) -> i64 {
    let mut core = ReactorCore::new();
    for command in commands {
        if command.inside_init() {
            core.execute_command(&command.restrict());
        }
    }
    core.count_on()
}

#[aoc(day22, part2)]
fn part2(commands: &[Command]) -> i64 {
    let mut core = ReactorCore::new();
    for command in commands {
        core.execute_command(command);
    }
    core.count_on()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small_test_center() {
        let input = parse_commands("on x=-1..2,y=-1..1,z=-1..1\noff x=0..0,y=0..0,z=0..0");
        assert_eq!(part1(&input), 35);

        let input = parse_commands(
            r"on x=-1..2,y=-1..1,z=-1..1
off x=0..0,y=0..0,z=0..0
on x=0..0,y=0..0,z=0..0",
        );
        assert_eq!(part1(&input), 36);

        let input = parse_commands(
            r"on x=-1..2,y=-1..1,z=-1..1
off x=0..0,y=0..0,z=0..0
on x=-1..2,y=-1..1,z=-1..1",
        );
        assert_eq!(part1(&input), 36);
    }

    #[test]
    fn small_test_overlap() {
        let input = parse_commands("on x=-1..2,y=-1..1,z=-1..1\noff x=0..0,y=0..0,z=0..3");
        assert_eq!(part1(&input), 34);
    }

    #[test]
    fn small_example() {
        let input = parse_commands(
            r"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10",
        );

        assert_eq!(part2(&input), 39);
    }

    #[test]
    fn small_test_corner() {
        // (0, 0, 0), (0, 0, 1),
        // (0, 1, 0), (0, 1, 1),
        // (1, 0, 0), (1, 0, 1),
        // (1, 1, 0), (1, 1, 1),
        let input = parse_commands("on x=-1..1,y=-1..1,z=-1..1\noff x=0..2,y=0..2,z=0..2");
        assert_eq!(part1(&input), 19);

        let input = parse_commands("on x=-1..1,y=-1..1,z=-1..1\non x=0..2,y=0..2,z=0..2");
        assert_eq!(part1(&input), 46);
    }

    #[test]
    fn test1() {
        let input = parse_commands(
            r"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682",
        );

        assert_eq!(part1(&input), 590784);
    }

    #[test]
    fn test2() {
        let input = r"on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";

        assert_eq!(part2(&parse_commands(input)), 2758514936282235);
    }
}
