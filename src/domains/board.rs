#[derive(Clone)]
pub struct Board {
    board: Vec<usize>,
    i: usize,
    j: usize,
}

impl Board {
    // pub fn get(&self, i: usize) -> usize {
    //     self.board[i]
    // }

    pub fn get_board(&self) -> Vec<usize> {
        self.board.clone()
    }

    pub fn get_ij(&self, i: usize, j: usize) -> usize {
        self.board[self.ij_to_pos(i, j)]
    }

    pub fn get_i(&self) -> usize {
        self.i
    }

    pub fn get_j(&self) -> usize {
        self.j
    }

    // pub fn len(&self) -> usize {
    //     self.board.len()
    // }

    pub fn run_length(&self, mut i: isize, mut j: isize, dir_i: isize, dir_j: isize) -> isize {
        // Get piece at current location.
        let p = self.get_ij(i as usize, j as usize);
        let mut n = 1;
        let max_i: isize = (self.get_i() - 1) as isize;
        let max_j: isize = (self.get_j() - 1) as isize;
        // Search in direction.
        loop {
            let next_i = i + dir_i;
            if (next_i < 0) || (next_i > max_i) {
                break;
            }
            let next_j = j + dir_j;
            if (next_j < 0) || (next_j > max_j) {
                break;
            }
            if self.get_ij(next_i as usize, next_j as usize) != p {
                break;
            }
            i += dir_i;
            j += dir_j;
            n += 1;
        }
        n
    }

    pub fn new(i: usize, j: usize) -> Board {
        Board {
            board: vec![0; i * j],
            i,
            j,
        }
    }

    pub fn put_ij(&mut self, i: usize, j: usize, p: usize) {
        let pos = self.ij_to_pos(i, j);
        self.board[pos] = p;
    }

    pub fn reset(&mut self) {
        for i in 0..self.board.len() {
            self.board[i] = 0;
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for ii in 0..self.i {
            for jj in 0..self.j {
                s.push_str(&format!("{}", self.get_ij(ii, jj)));
            }
            s.push_str("\n");
        }
        s
    }

    fn ij_to_pos(&self, i: usize, j: usize) -> usize {
        i * self.j + j
    }
}
