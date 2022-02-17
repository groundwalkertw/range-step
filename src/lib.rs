use std::ops::Range;


macro_rules! impl_range {
    ($($t: ty)*) => {
        $(
            impl From<Range<$t>> for ERange<$t>
            {
                fn from(range: Range<$t>) -> Self {
                    if range.start < range.end {
                        crate::ERange::<$t>::from_range(range, 1 as $t)
                    } else {
                        crate::ERange::<$t>::from_range(range, -1 as $t)
                    }
                }
            }

        )*
    };
}
impl_range!(i8 i16 i32 i64 i128 isize f32 f64);

enum IterAB<T, A, B> 
    where A: Iterator<Item = T>, B: DoubleEndedIterator<Item = T>
{
    IterA(A),
    IterB(B),
}

impl<T, A, B> Iterator for IterAB<T, A, B> 
    where A: Iterator<Item = T>, B: DoubleEndedIterator<Item = T>
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterAB::IterA(iter_a) => iter_a.next(),
            IterAB::IterB(iter_b) => iter_b.next(),
        }
    }
}

macro_rules! range_int {
    ($start: expr, $end: expr) => {
        if $start <= $end {
            crate::IterAB::IterA($start..$end)
        } else {
            crate::IterAB::IterB((($end + 1)..($start + 1)).rev())
        }
    };
}

#[macro_export]
macro_rules! from_range {
    ($range: expr, $step: expr) => {
        crate::ERange::from_range($range, $step)
    };
    ($range: expr) => {
        crate::ERange::from($range)
    }
}


#[macro_export]
macro_rules! range {
    ($end: expr) => {
        ..end
    };
    ($start: expr, $end: expr) => {
        crate::ERange::from($start..$end)        
    };
    ($start: expr, $end: expr, $step: expr) => {
        crate::ERange::new($start, $end, $step)
    }
}



pub struct ERange<T> 
    where T: core::ops::Add<Output = T> + PartialEq + PartialOrd + Clone + Copy
{   
    end: T,
    step: T,
    now: T,
    direction: Direction
}
enum Direction {
    Plus,
    Minus,
    Stop,
}

impl<T> ERange<T> 
    where T: core::ops::Add<Output = T> + PartialEq + PartialOrd + Clone + Copy
{   
    #[inline]
    fn new(start: T, end: T, step: T) -> Self {
        let direction = if start <= end && start + step < end {
            Direction::Plus
        } else if start >= end && start + step > end {
            Direction::Minus
        } else {
            Direction::Stop
        };

        ERange {
            end,
            step,
            now: start,
            direction,
        }
    }
    #[inline]
    fn from_range(range: Range<T>, step: T) -> Self {
        Self::new(range.start, range.end, step)
    }
}

impl<T> Iterator for ERange<T> 
    where T: std::ops::Add<Output = T> + PartialEq + PartialOrd + Clone + Copy
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {        
        let return_now = self.now;
        self.now = self.now + self.step;

        match self.direction {
            Direction::Plus => {                                
                if self.now >= self.end {
                    self.direction = Direction::Stop;
                }
                Some(return_now)
            },
            Direction::Minus => {
                if self.now <= self.end {
                    self.direction = Direction::Stop;
                }
                Some(return_now)
            },
            Direction::Stop => {
                None
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result: Vec<i32> = from_range!(5..1).collect();
        assert_eq!(vec![5, 4, 3, 2], result);
    }
    #[test]
    fn it_works_2() {
        let result: Vec<i32> = range!(5, 1).collect();
        assert_eq!(vec![5, 4, 3, 2], result);
    }
    #[test]
    fn it_works_3() {
        let result: Vec<i32> = range!(5, 1, -2).collect();
        assert_eq!(vec![5, 3], result);
    }
    #[test]
    fn it_works_4() {
        let result: Vec<i32> = range_int!(1, 5).collect();
        assert_eq!(vec![1,2,3,4], result);
    }
    #[test]
    fn it_works_5() {
        let result: Vec<i32> = range_int!(5, 1).collect();
        assert_eq!(vec![5,4,3,2], result);
    }
}
