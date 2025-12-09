use std::ops::Index;

pub fn part_1(input: Vec<String>) -> u32 {
    let grid = Grid::new(input);
    let mut answer  =  0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
        }
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

fn get_neighbors_coords(grid: &Grid, x: usize, y: usize) -> Vec<(usize, usize)> {
    get_neighbors_iter(grid, x, y).collect()
}

fn get_neighbors_iter(grid: &Grid, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    vec![
        (-1i8, -1i8), (-1i8, 0i8), (-1i8, 1i8),
        (0i8, -1i8), (0i8, 1i8),
        (1i8, -1i8), (1i8, 0i8), (1i8, 1i8),
    ].into_iter()
      .map(move |(ny, nx)| (y as i8 + ny, x as i8 + nx))
      .filter(|(y, x)|
        *y >= 0 && *y < grid.height() as i8 &&
          *x >= 0 && *x < grid.width() as i8
      )
      .map(|(y, x)| (y as usize, x as usize))
}

pub fn get_neighbors(grid: &Grid, x: usize, y: usize) -> impl Iterator<Item=u8>{
    get_neighbors_iter(grid, x, y).map(|(y, x)| {
        grid[y][x]
    })
}

pub fn part_2(input: Vec<String>) -> u64 {
    0
}




#[cfg(test)]
mod day04_test {
    use rstest::rstest;
    use crate::day04::{create_row, get_neighbors, get_neighbors_coords, Grid, part_1, part_2};
    use crate::get_sample_input;
    #[test]
    #[ignore]
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
        let neighbors = get_neighbors_coords(&grid, pos.1, pos.0);
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
    #[ignore]
    fn sample_test_2() {
        let input = get_sample_input("day03.txt");
        let answer = part_2(input);
        assert_eq!(answer, 3121910778619);
    }

}
