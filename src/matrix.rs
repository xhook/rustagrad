use std::ops;

#[derive(Clone)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    m: Vec<T>
}

impl Matrix<f32> {
    pub fn new(rows: usize, cols: usize) -> Matrix<f32> {
        Matrix {
            rows: rows,
            cols: cols,
            m: Vec::with_capacity(rows * cols)
        }
    }

    pub fn zeros(rows: usize, cols: usize) -> Matrix<f32> {
        let size = rows * cols;
        let mut m: Vec<f32> = Vec::with_capacity(size);
        for _i in 0..size {
            m.push(0.0f32);
        }
        Matrix {
            rows: rows,
            cols: cols,
            m: m
        }
    }
    
    pub fn diag(rows: usize, cols: usize) -> Matrix<f32> {
        let size = rows * cols;
        let mut m: Vec<f32> = Vec::with_capacity(size);
        for i in 0..size {
            m.push(
                if i / cols == i % cols { 1.0f32 } 
                else { 0.0f32 }
            );
        }
        Matrix {
            rows: rows,
            cols: cols,
            m: m
        }
    }
    
    pub fn add(mut self, other: Matrix<f32>) -> Self {
        if self.cols != other.cols || self.rows != other.rows {
            panic!("Dimensions of matrices do not match self({}, {}), other({}, {})", self.rows, self.cols, other.rows, other.cols) 
        }
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.m[i*self.cols+j] += other.m[i*self.cols+j]
            }
        }
        self
    }
}

impl std::fmt::Display for Matrix<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols-1 {
                let res = write!(f, "{}, ", self.m[i*self.cols+j]);
                if !res.is_ok() {
                    return res;
                }
            }
            let res = write!(f, "{}\n", self.m[(i+1)*self.cols-1]);
            if !res.is_ok() {
                return res;
            }
        }
        Ok(())
    }
}

impl ops::Add<Matrix<f32>> for Matrix<f32> {
    type Output = Matrix<f32>;

    fn add(self, _rhs: Matrix<f32>) -> Matrix<f32> {
        self.add(_rhs)
    }
}