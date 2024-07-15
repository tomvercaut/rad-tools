use crate::order::MemoryOrder;

/// The `IndexError` enum represents the possible errors that can occur while indexing.
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum IndexError {
    #[error("Length of the index [{0}] doesn't match with the number of dimensions [{1}].")]
    IndexDimensionLength(usize, usize),
    #[error("Index [{0}] exceeds size [{1}]")]
    IndexOutOfBound(usize, usize),
}

pub type Result<T> = std::result::Result<T, IndexError>;

/// Converts a multidimensional index to a linear index.
///
/// This function takes a multidimensional index `idx` in row-major and a dimensions vector `dims`,
/// and computes the corresponding linear index offset. The dimensions vector
/// specifies the size of each dimension in the multidimensional array. The
/// function returns the computed multidimensional index.
///
/// More information on row-major can be found on [Wikipedia](https://en.wikipedia.org/wiki/Row-_and_column-major_order).
///
/// # Arguments
///
/// * `idx` - The multidimensional index in row-major to convert.
/// * `dims` - The dimensions vector specifying the size of each dimension.
///
/// # Example
///
/// ```
/// use rad_tools_threed::index::row_major;
///
/// assert_eq!(47, row_major(&[2,1,2], &[3,4,5]).unwrap());
/// assert_eq!(48, row_major(&[2,1,3], &[3,4,5]).unwrap());
/// assert_eq!(49, row_major(&[2,1,4], &[3,4,5]).unwrap());
/// ```
///
/// # Errors
///
/// This function returns an error if the length of the input index doesn't
/// match the length of the dimensions vector, or if any of the indices is out
/// of bounds for its corresponding dimension.
///
/// If the length of the index and dimensions vector is not equal, an
/// `IndexError::IndexDimensionLength` error is returned.
///
/// If any of the indices is greater than or equal to its corresponding
/// dimension, an `IndexError::IndexOutOfBound` error is returned.
#[allow(clippy::needless_range_loop)]
pub fn row_major(idx: &[usize], dims: &[usize]) -> Result<usize> {
    let n = idx.len();
    if n != dims.len() {
        return Err(IndexError::IndexDimensionLength(n, dims.len()));
    }
    let mut index = 0;
    for k in 0..n {
        if idx[k] >= dims[k] {
            return Err(IndexError::IndexOutOfBound(idx[k], dims[k]));
        }
        let mut m = 1;
        for l in (k + 1)..n {
            m *= dims[l];
        }
        index += idx[k] * m;
    }
    Ok(index)
}

/// Converts a multidimensional index to a linear index.
///
/// This function takes a multidimensional index `idx` in column-major order and a dimensions vector `dims`,
/// and computes the corresponding linear index offset. The dimensions vector
/// specifies the size of each dimension in the multidimensional array. The
/// function returns the computed multidimensional index.
///
/// More information on column-major can be found on [Wikipedia](https://en.wikipedia.org/wiki/Row-_and_column-major_order).
///
/// # Arguments
///
/// * `idx` - The multidimensional index in column-major to convert.
/// * `dims` - The dimensions vector specifying the size of each dimension.
///
/// # Example
///
/// ```
/// use rad_tools_threed::index::column_major;
///
/// assert_eq!(27, column_major(&[0,1,2], &[3,4,5]).unwrap());
/// assert_eq!(28, column_major(&[1,1,2], &[3,4,5]).unwrap());
/// assert_eq!(29, column_major(&[2,1,2], &[3,4,5]).unwrap());
/// ```
///
/// # Errors
///
/// This function returns an error if the length of the input index doesn't
/// match the length of the dimensions vector, or if any of the indices is out
/// of bounds for its corresponding dimension.
///
/// If the length of the index and dimensions vector is not equal, an
/// `IndexError::IndexDimensionLength` error is returned.
///
/// If any of the indices is greater than or equal to its corresponding
/// dimension, an `IndexError::IndexOutOfBound` error is returned.
#[allow(clippy::needless_range_loop)]
pub fn column_major(idx: &[usize], dims: &[usize]) -> Result<usize> {
    let n = idx.len();
    if n != dims.len() {
        return Err(IndexError::IndexDimensionLength(n, dims.len()));
    }
    let mut index = 0;
    for k in 0..n {
        if idx[k] >= dims[k] {
            return Err(IndexError::IndexOutOfBound(idx[k], dims[k]));
        }
        let mut m = 1;
        for l in 0..k {
            m *= dims[l];
        }
        index += idx[k] * m;
    }
    Ok(index)
}

/// The `MemoryIndex` trait provides an interface for accessing data in memory using multidimensional indices.
pub trait MemoryIndex {
    /// Retrieves the value at the given multidimensional index from the underlying data structure.
    ///
    /// # Arguments
    ///
    /// * `idx` - A slice of usize values representing the indices of the value to retrieve.
    ///
    /// # Returns
    ///
    /// * `Result<usize>` - The value at the given index if it exists, or an error if the index is out of range.
    fn get(&self, idx: &[usize]) -> Result<usize>;

    /// A method that returns a reference to an array of `usize` that represents the dimensions of a multidimensional matrix or array.
    ///
    /// # Returns
    ///
    /// A reference to an array of `usize` representing the dimensions.
    fn dimensions(&self) -> &[usize];
    
    /// Returns the memory order used to compute the index.
    ///
    /// # Returns
    ///
    /// The memory order used to compute the index.
    ///
    /// # Example
    ///
    /// ```
    /// use rad_tools_threed::index::{MemoryIndex, RowMajorIndex};
    /// use rad_tools_threed::order::MemoryOrder;
    ///
    /// let index = RowMajorIndex::new(&[10,20]);
    /// assert_eq!(MemoryOrder::RowMajor, index.order());
    /// ```
    fn order(&self) -> MemoryOrder;
}


/// Represents a row-major index for multidimensional arrays.
///
/// This struct is used to represent the index of an element in a multidimensional array accessed in row-major order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RowMajorIndex {
    dims: Vec<usize>,
}

impl RowMajorIndex {
    /// Constructs a new instance of `RowMajorIndex` with the given dimensions.
    ///
    /// # Arguments
    ///
    /// * `dims` - A slice of the dimensions of the array
    ///
    /// # Returns
    ///
    /// A new instance of `RowMajorIndex` with the given dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// use rad_tools_threed::index::RowMajorIndex;
    ///
    /// let index = RowMajorIndex::new(&[10,20]);
    /// ```
    pub fn new(dims: &[usize]) -> Self {
        Self {
            dims: dims.to_vec(),
        }
    }
    
    /// Returns the number of dimensions in the Index.
    /// 
    /// # Example
    ///
    /// ```
    /// use rad_tools_threed::index::RowMajorIndex;
    ///
    /// let index = RowMajorIndex::new(&[10,20]);
    /// assert_eq!(2, index.len());
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.dims.len()
    }

    /// Checks if the index has no dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rad_tools_threed::index::RowMajorIndex;
    ///
    /// let index = RowMajorIndex::new(&[10, 20]);
    /// assert_eq!(false, index.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.dims.is_empty()
    }
}

impl MemoryIndex for RowMajorIndex {
    fn get(&self, idx: &[usize]) -> Result<usize> {
        row_major(idx, self.dimensions())
    }

    #[inline]
    fn dimensions(&self) -> &[usize] {
        &self.dims
    }

    #[inline]
    fn order(&self) -> MemoryOrder {
        MemoryOrder::RowMajor
    }
}

/// Represents a column-major index for multidimensional arrays.
///
/// This struct is used to represent the index of an element in a multidimensional array accessed in column-major order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnMajorIndex {
    dims: Vec<usize>,
}

impl ColumnMajorIndex {
    /// Constructs a new instance of `ColumnMajorIndex` with the given dimensions.
    ///
    /// # Arguments
    ///
    /// * `dims` - A slice of the dimensions of the array
    ///
    /// # Returns
    ///
    /// A new instance of `ColumnMajorIndex` with the given dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// use rad_tools_threed::index::ColumnMajorIndex;
    ///
    /// let index = ColumnMajorIndex::new(&[10,20]);
    /// ``` 
    pub fn new(dims: &[usize]) -> Self {
        Self {
            dims: dims.to_vec(),
        }
    }
    
    /// Returns the number of dimensions in the Index.
    ///
    /// # Example
    ///
    /// ```
    /// use rad_tools_threed::index::ColumnMajorIndex;
    ///
    /// let index = ColumnMajorIndex::new(&[10,20]);
    /// assert_eq!(2, index.len());
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.dims.len()
    }

    /// Checks if the index has no dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rad_tools_threed::index::ColumnMajorIndex;
    ///
    /// let index = ColumnMajorIndex::new(&[10, 20]);
    /// assert_eq!(false, index.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.dims.is_empty()
    }
}

impl MemoryIndex for ColumnMajorIndex {
    fn get(&self, idx: &[usize]) -> Result<usize> {
        column_major(idx, self.dimensions())
    }

    #[inline]
    fn dimensions(&self) -> &[usize] {
        &self.dims
    }

    #[inline]
    fn order(&self) -> MemoryOrder {
        MemoryOrder::ColumnMajor
    }
}


#[cfg(test)]
mod test {
    use super::{ColumnMajorIndex, IndexError, MemoryIndex, RowMajorIndex};

    fn init() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Trace)
            .try_init();
    }

    fn row_major_test_slices() -> &'static [&'static [usize]] {
        &[
            &[0usize, 0, 0],
            &[0, 0, 1],
            &[0, 0, 2],
            &[0, 1, 0],
            &[0, 1, 1],
            &[0, 1, 2],
            &[1, 0, 0],
            &[1, 0, 1],
            &[1, 0, 2],
            &[2, 0, 2],
            &[2, 1, 2],
        ]
    }

    fn column_major_test_slices() -> &'static [&'static [usize]] {
        &[
            &[0usize, 0, 0],
            &[1, 0, 0],
            &[2, 0, 0],
            &[0, 1, 0],
            &[1, 1, 0],
            &[2, 1, 0],
            &[0, 0, 1],
            &[1, 0, 1],
            &[2, 0, 1],
            &[2, 1, 1],
            &[2, 1, 2],
        ]
    }

    #[test]
    fn row_major() {
        let dims = &[3, 4, 5];
        let v = row_major_test_slices();
        let e = [0, 1, 2, 5, 6, 7, 20, 21, 22, 42, 47];
        let n = e.len();
        assert_eq!(n, v.len());
        for i in 0..n {
            let idx = super::row_major(v[i], dims).unwrap();
            assert_eq!(e[i], idx);
        }
    }

    #[test]
    fn row_major_err_index_dimension_length() {
        let r = super::row_major(&[0, 1], &[3, 4, 5]);
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert_eq!(IndexError::IndexDimensionLength(2, 3), e);
    }

    #[test]
    fn row_major_err_out_of_bound() {
        let dims = &[3, 4, 5];
        let mut r = super::row_major(&[3, 0, 0], dims);
        assert!(r.is_err());
        let mut e = r.unwrap_err();
        assert_eq!(IndexError::IndexOutOfBound(3, 3), e);

        r = super::row_major(&[0, 4, 0], dims);
        assert!(r.is_err());
        e = r.unwrap_err();
        assert_eq!(IndexError::IndexOutOfBound(4, 4), e);

        r = super::row_major(&[0, 0, 5], dims);
        assert!(r.is_err());
        e = r.unwrap_err();
        assert_eq!(IndexError::IndexOutOfBound(5, 5), e);
    }

    #[test]
    fn column_major() {
        init();
        let dims = &[3, 4, 5];
        let v = column_major_test_slices();
        let e = [0, 1, 2, 3, 4, 5, 12, 13, 14, 17, 29];
        let n = e.len();
        assert_eq!(n, v.len());
        for i in 0..n {
            let idx = super::column_major(v[i], dims).unwrap();
            assert_eq!(e[i], idx);
        }
    }


    #[test]
    fn column_major_err_index_dimension_length() {
        let r = super::column_major(&[0, 1], &[3, 4, 5]);
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert_eq!(IndexError::IndexDimensionLength(2, 3), e);
    }

    #[test]
    fn column_major_err_out_of_bound() {
        let dims = &[3, 4, 5];
        let mut r = super::column_major(&[3, 0, 0], dims);
        assert!(r.is_err());
        let mut e = r.unwrap_err();
        assert_eq!(IndexError::IndexOutOfBound(3, 3), e);

        r = super::column_major(&[0, 4, 0], dims);
        assert!(r.is_err());
        e = r.unwrap_err();
        assert_eq!(IndexError::IndexOutOfBound(4, 4), e);

        r = super::column_major(&[0, 0, 5], dims);
        assert!(r.is_err());
        e = r.unwrap_err();
        assert_eq!(IndexError::IndexOutOfBound(5, 5), e);
    }

    #[test]
    fn row_major_index() {
        let index = RowMajorIndex::new(&[3, 4, 5]);
        let v = row_major_test_slices();
        let e = [0, 1, 2, 5, 6, 7, 20, 21, 22, 42, 47];
        let n = e.len();
        assert_eq!(n, v.len());
        for i in 0..n {
            let idx = index.get(v[i]).unwrap();
            assert_eq!(e[i], idx);
        }
    }

    #[test]
    fn row_major_index_err_index_dimension_length() {
        let index = RowMajorIndex::new(&[3, 4, 5]);
        let r = index.get(&[0, 1]);
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert_eq!(IndexError::IndexDimensionLength(2, 3), e);
    }

    #[test]
    fn row_major_index_err_out_of_bound() {
        let index = RowMajorIndex::new(&[3, 4, 5]);
        let mut r = index.get(&[3, 0, 0]);
        assert!(r.is_err());
        let mut e = r.unwrap_err();
        assert_eq!(IndexError::IndexOutOfBound(3, 3), e);

        r = index.get(&[0, 4, 0]);
        assert!(r.is_err());
        e = r.unwrap_err();
        assert_eq!(IndexError::IndexOutOfBound(4, 4), e);

        r = index.get(&[0, 0, 5]);
        assert!(r.is_err());
        e = r.unwrap_err();
        assert_eq!(IndexError::IndexOutOfBound(5, 5), e);
    }


    #[test]
    fn column_major_index() {
        let index = ColumnMajorIndex::new(&[3, 4, 5]);
        let v = column_major_test_slices();
        let e = [0, 1, 2, 3, 4, 5, 12, 13, 14, 17, 29];
        let n = e.len();
        assert_eq!(n, v.len());
        for i in 0..n {
            let idx = index.get(v[i]).unwrap();
            assert_eq!(e[i], idx);
        }
    }


    #[test]
    fn column_major_index_err_index_dimension_length() {
        let index = ColumnMajorIndex::new(&[3, 4, 5]);
        let r = index.get(&[0, 1]);
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert_eq!(IndexError::IndexDimensionLength(2, 3), e);
    }

    #[test]
    fn column_major_index_err_out_of_bound() {
        let index = ColumnMajorIndex::new(&[3, 4, 5]);
        let mut r = index.get(&[3, 0, 0]);
        assert!(r.is_err());
        let mut e = r.unwrap_err();
        assert_eq!(IndexError::IndexOutOfBound(3, 3), e);

        r = index.get(&[0, 4, 0]);
        assert!(r.is_err());
        e = r.unwrap_err();
        assert_eq!(IndexError::IndexOutOfBound(4, 4), e);

        r = index.get(&[0, 0, 5]);
        assert!(r.is_err());
        e = r.unwrap_err();
        assert_eq!(IndexError::IndexOutOfBound(5, 5), e);
    }
}
