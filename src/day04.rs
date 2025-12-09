use std::ops::Index;

pub fn part_1(input: Vec<String>) -> u32 {
    let grid = Grid::new(input);
    let mut answer  =  0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid[y][x] == 0 { continue }
            let papers = get_neighbors(&grid, x, y).map(|n| n as u32).sum::<u32>();
            if papers < 4 {
                answer += 1;
            }
        }
    }
    answer
}

pub fn part_2(input: Vec<String>) -> u64 {
    let mut grid = Grid::new(input);
    let mut answer  =  0;
    loop {
        let mut acc = 0;
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if grid[y][x] == 0 { continue }
                let papers = get_neighbors(&grid, x, y).map(|n| n as u32).sum::<u32>();
                if papers < 4 {
                    acc += 1;
                    grid.remove((x, y));
                }
            }
        }
        if acc == 0 { break; }
        answer += acc
    }
    answer
}

pub struct Grid {
    data: Vec<Vec<u8>>,
    height: usize,
    width: usize
}

impl Grid {
    pub fn new(input: Vec<String>) -> Self {
        let data  = create_grid(input);
        Grid {
            height: data.len(),
            width: data[0].len(),
            data
        }
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn remove(&mut self, (x, y): (usize, usize)) {
        self.data[y][x] = 0;
    }
}

impl Index<usize> for Grid {
    type Output = Vec<u8>;

    fn index(&self, index: usize) -> &Self::Output {
       &self.data[index]
    }
}

fn create_row(row: &str) -> Vec<u8> {
    row.chars().map(|c| { if c == '@'{
        return 1;
    }
    return 0
    }).collect()
}

fn create_grid(input: Vec<String>) -> Vec<Vec<u8>> {
    input.iter().map(|i| create_row(i)).collect()
}

fn get_neighbors_iter(grid: &Grid, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    vec![
        (-1i32, -1i32), (-1i32, 0i32), (-1i32, 1i32),
        (0i32, -1i32), (0i32, 1i32),
        (1i32, -1i32), (1i32, 0i32), (1i32, 1i32),
    ].into_iter()
      .map(move |(ny, nx)| (y as i32 + ny, x as i32 + nx))
      .filter(|(y, x)|
        *y >= 0 && *y < grid.height() as i32 &&
          *x >= 0 && *x < grid.width() as i32
      )
      .map(|(y, x)| (y as usize, x as usize))
}

pub fn get_neighbors(grid: &Grid, x: usize, y: usize) -> impl Iterator<Item=u8>{
    get_neighbors_iter(grid, x, y).map(|(y, x)| {
        grid[y][x]
    })
}



#[cfg(test)]
mod day04_test {
    use rstest::rstest;
    use crate::day04::{create_row, get_neighbors, get_neighbors_iter, Grid, part_1, part_2};
    use crate::get_sample_input;

    #[test]
    fn sample_test_1() {
        let input = get_sample_input("day04.txt");
        let answer = part_1(input);
        assert_eq!(answer, 13);
    }

    #[test]
    fn creates_row() {
        let row = create_row(".");
        assert_eq!(row, vec![0])
    }

    #[test]
    fn creates_grid() {
        let input = vec![".".to_string(), ".".to_string()];
        let grid = Grid::new(input);
        assert_eq!(grid[0][0], 0);
        assert_eq!(grid[1][0], 0);
    }

    #[test]
    fn creates_grid_with_paper() {
        let input = vec!["@".to_string(), "@".to_string()];
        let grid = Grid::new(input);
        assert_eq!(grid[0][0], 1);
        assert_eq!(grid[1][0], 1);
    }

    #[test]
    fn parses_sample_input() {
        let input = get_sample_input("day04.txt");
        let grid = Grid::new(input);
        assert_eq!(grid[0][0], 0);
        assert_eq!(grid[0][1], 0);
        assert_eq!(grid[0][2], 1);
        assert_eq!(grid[0][3], 1);
        assert_eq!(grid[1][0], 1);
        assert_eq!(grid[9][9], 0);
    }

    #[rstest]
    #[case((0, 0), vec![(0, 1), (1, 0), (1, 1)])]
    #[case((9, 9), vec![(8, 8), (8, 9), (9, 8)])]
    #[case((0, 9), vec![(0, 8), (1, 8), (1, 9)])]
    #[case((9, 0), vec![(8, 0), (8, 1), (9, 1)])]
    #[case((3, 3), vec![(2, 2), (2, 3), (2, 4), (3, 2), (3, 4), (4, 2), (4, 3), (4, 4)])]
    fn getting_neighbors_coords(
        #[case] pos: (usize, usize),
        #[case] expected_neighbors: Vec<(usize, usize)>
    ) {
        let input = get_sample_input("day04.txt");
        let grid = Grid::new(input);
        let neighbors: Vec<_> = get_neighbors_iter(&grid, pos.1, pos.0).collect();
        assert_eq!(neighbors, expected_neighbors)
    }

    #[rstest]
    #[case((0, 0), vec![0, 1, 1])]
    #[case((3, 3), vec![1, 1, 1, 1, 1, 0, 1, 1])]
    fn getting_neighbors(
        #[case] pos: (usize, usize),
        #[case] expected_neighbors: Vec<u8>
    ) {
        let input = get_sample_input("day04.txt");
        let grid = Grid::new(input);
        let neighbors: Vec<u8> = get_neighbors(&grid, pos.1, pos.0).collect();
        assert_eq!(neighbors, expected_neighbors)
    }

    #[test]
    fn sample_test_2() {
        let input = get_sample_input("day04.txt");
        let answer = part_2(input);
        assert_eq!(answer, 43);
    }

    #[test]
    fn removes_from_grid() {
        let input = get_sample_input("day04.txt");
        let mut grid = Grid::new(input);
        assert_eq!(grid[0][3], 1);
        grid.remove((3, 0));
        assert_eq!(grid[0][3], 0);
    }

}
