use std::io::{self, Write};

pub struct Unbounded;

pub struct Bounded {
    bound: usize,
    delims: (char, char),
}

pub trait ProgressDisplay: Sized {
    fn display(&self) -> Result<(), io::Error>;
}

impl<Iter> ProgressDisplay for Progress<Iter, Bounded> {
    fn display(&self) -> io::Result<()> {
        print!(
            "\r{}{:width$}{}",
            self.bound.delims.0,
            "*".repeat(self.i),
            self.bound.delims.1,
            width = self.bound.bound,
        );
        io::stdout().flush()?;
        Ok(())
    }
}

impl<Iter> ProgressDisplay for Progress<Iter, Unbounded> {
    fn display(&self) -> io::Result<()> {
        print!("\r{}", "*".repeat(self.i));
        io::stdout().flush()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Progress<Iter, Bound = Unbounded> {
    iter: Iter,
    i: usize,
    bound: Bound,
}

impl<Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Progress {
            iter,
            i: 0,
            bound: Unbounded,
        }
    }
}

impl<Iter> Progress<Iter, Bounded> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

pub trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where
    Iter: Iterator,
    Progress<Iter, Bound>: ProgressDisplay,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let _ = self.display();
        self.i += 1;
        self.iter.next()
    }
}

impl<Iter> Progress<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    pub fn with_bound(self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };
        Progress {
            iter: self.iter,
            i: self.i,
            bound,
        }
    }
}
