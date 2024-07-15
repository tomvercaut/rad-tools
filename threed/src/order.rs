/// An enumeration representing the memory order.
///
/// This enumeration has two variants: `RowMajor` and `ColumnMajor`.
/// It is used to determine the memory layout of a matrix or multidimensional array.
///
/// # Examples
///
/// ```
/// use rad_tools_threed::order::MemoryOrder;
///
/// let row_major = MemoryOrder::RowMajor;
/// let column_major = MemoryOrder::ColumnMajor;
///
/// assert_eq!(row_major, MemoryOrder::RowMajor);
/// assert_eq!(column_major, MemoryOrder::ColumnMajor);
/// ```
///
/// # Debugging
///
/// The `MemoryOrder` enumeration derives from `Debug`, allowing it to be easily printed for
/// debugging purposes.
///
/// ```
/// use rad_tools_threed::order::MemoryOrder;
///
/// let row_major = MemoryOrder::RowMajor;
/// let column_major = MemoryOrder::ColumnMajor;
///
/// println!("{:?}", row_major);        // prints "RowMajor"
/// println!("{:?}", column_major);     // prints "ColumnMajor"
/// ```
///
/// # Equivalence
///
/// The `MemoryOrder` enumeration derives from `PartialEq` and `Eq`, allowing for comparison
/// between `MemoryOrder` values.
///
/// ```
/// use rad_tools_threed::order::MemoryOrder;
///
/// let row_major_a = MemoryOrder::RowMajor;
/// let row_major_b = MemoryOrder::RowMajor;
/// let column_major = MemoryOrder::ColumnMajor;
///
/// assert_eq!(row_major_a, row_major_b);
/// assert_ne!(row_major_a, column_major);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryOrder {
    RowMajor,
    ColumnMajor,
}
