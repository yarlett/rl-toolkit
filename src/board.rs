pub struct Board {
    board: Vec<usize>,
    i: usize,
    j: usize,
}

impl Board {
    pub fn get(&self, i: usize) -> usize {
        self.board[i]
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

    pub fn len(&self) -> usize {
        self.board.len()
    }

    pub fn longest_col(&self, i: usize, j: usize) -> usize {
        // Get piece at current location.
        let p = self.get_ij(i, j);
        let mut n = 1;
        // Search up.
        let mut ii = i;
        loop {
            if ii == 0 {
                break;
            };
            if self.get_ij(ii - 1, j) == p {
                n += 1;
                ii -= 1;
            } else {
                break;
            }
        }
        // Search down.
        let mut ii = i;
        loop {
            if ii == (self.get_i() - 1) {
                break;
            };
            if self.get_ij(ii + 1, j) == p {
                n += 1;
                ii += 1;
            } else {
                break;
            }
        }
        n
    }

    pub fn longest_diag_1(&self, i: usize, j: usize) -> usize {
        // Get piece at current location.
        let p = self.get_ij(i, j);
        let mut n = 1;
        // Search --.
        let mut ii = i;
        let mut jj = j;
        loop {
            if (ii == 0) | (jj == 0) {
                break;
            };
            if self.get_ij(ii - 1, jj - 1) == p {
                n += 1;
                ii -= 1;
                jj -= 1;
            } else {
                break;
            }
        }
        // Search ++.
        let mut ii = i;
        let mut jj = j;
        loop {
            if (ii == (self.get_i() - 1)) | (jj == (self.get_j() - 1)) {
                break;
            };
            if self.get_ij(ii + 1, jj + 1) == p {
                n += 1;
                ii += 1;
                jj += 1
            } else {
                break;
            }
        }
        n
    }

    pub fn longest_diag_2(&self, i: usize, j: usize) -> usize {
        // Get piece at current location.
        let p = self.get_ij(i, j);
        let mut n = 1;
        // Search +-.
        let mut ii = i;
        let mut jj = j;
        loop {
            if (ii == (self.get_i() - 1)) | (jj == 0) {
                break;
            };
            if self.get_ij(ii + 1, jj - 1) == p {
                n += 1;
                ii += 1;
                jj -= 1;
            } else {
                break;
            }
        }
        // Search -+.
        let mut ii = i;
        let mut jj = j;
        loop {
            if (ii == 0) | (jj == (self.get_j() - 1)) {
                break;
            };
            if self.get_ij(ii - 1, jj + 1) == p {
                n += 1;
                ii -= 1;
                jj += 1
            } else {
                break;
            }
        }
        n
    }

    pub fn longest_row(&self, i: usize, j: usize) -> usize {
        // Get piece at current location.
        let p = self.get_ij(i, j);
        let mut n = 1;
        // Search left.
        let mut jj = j;
        loop {
            if jj == 0 {
                break;
            };
            if self.get_ij(i, jj - 1) == p {
                n += 1;
                jj -= 1;
            } else {
                break;
            }
        }
        // Search right.
        let mut jj = j;
        loop {
            if jj == (self.get_j() - 1) {
                break;
            };
            if self.get_ij(i, jj + 1) == p {
                n += 1;
                jj += 1;
            } else {
                break;
            }
        }
        n
    }

    pub fn new(i: usize, j: usize) -> Board {
        Board {
            board: vec![0; i * j],
            i: i,
            j: j,
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
