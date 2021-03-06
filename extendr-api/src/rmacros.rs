//!
//! rmacros - a set of macros to call actual R functions in a rusty way.
//!

/// R object encapsulation operator.
///
/// Shorthand for Robj::from(x).
///
/// Example:
/// ```
/// use extendr_api::*;
/// start_r();
/// let fred = r!(1);
/// assert_eq!(fred, Robj::from(1));
/// ```
#[macro_export]
macro_rules! r {
    ($e: expr) => {
        Robj::from($e)
    };
}

/// Concatenation operator.
///
/// Example:
/// ```
/// use extendr_api::*;
/// start_r();
/// let fred = c!(1, 2, 3);
/// assert_eq!(fred, Robj::from(&[1, 2, 3][..]));
/// ```
#[macro_export]
macro_rules! c {
    () => {
        lang!("c").eval_blind()
    };
    ($($rest: tt)*) => {
        lang!("c", $($rest)*).eval_blind()
    };
}

/// Create a vector with repeating elements.
///
/// Example:
/// ```
/// use extendr_api::*;
/// start_r();
/// let fred = rep!(1., 3);
/// assert_eq!(fred, Robj::from(&[1., 1., 1.][..]));
/// ```
#[macro_export]
macro_rules! rep {
    ($($rest: tt)*) => {
        lang!("rep", $($rest)*).eval_blind()
    };
}

/// Read a CSV file.
///
/// Example:
/// ```no_run
/// use extendr_api::*;
/// start_r();
/// let mydata = read_table!("mydata.csv").unwrap();
/// ```
#[macro_export]
macro_rules! read_table {
    ($($rest: tt)*) => {
        lang!("read.table", $($rest)*).eval()
    };
}

/// Create a list.
///
/// Example:
/// ```
/// use extendr_api::*;
/// start_r();
/// let mylist = list!(x=1, y=2);
/// assert_eq!(mylist, List(&[1.into(), 2.into()]));
/// ```
#[macro_export]
macro_rules! list {
    () => {
        lang!("list").eval_blind()
    };
    ($($rest: tt)*) => {
        lang!("list", $($rest)*).eval_blind()
    };
}

/// Create a dataframe.
///
/// Example:
/// ```
/// use extendr_api::*;
/// start_r();
/// let mydata = data_frame!(x=1, y=2);
/// assert_eq!(mydata, List(&[1.into(), 2.into()]));
/// ```
#[macro_export]
macro_rules! data_frame {
    () => {
        lang!("data.frame").eval_blind()
    };
    ($($rest: tt)*) => {
        lang!("data.frame", $($rest)*).eval_blind()
    };
}
