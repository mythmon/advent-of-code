use std::{
    fmt::Display,
    marker::PhantomData,
    time::{Duration, Instant},
    unimplemented,
};

/// A puzzle case that can be executed, comparing expected output to actual
/// output.
pub trait PuzzleCase: std::fmt::Debug + Sync + Send {
    fn name(&self) -> String;
    fn run(&self) -> PuzzleResult;
}

/// A set of puzzle cases and associated metadata
///
/// Importantly, this does not depend on the types of the input or outputs.
pub trait Puzzle: std::fmt::Debug + Sync + Send {
    fn name(&self) -> String;
    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>>;
}

/// A function to run a specific puzzle's code
///
/// In contrast to `Puzzle`, this trait contains the specific types for the
/// puzzle.
pub trait PuzzleRunner: std::fmt::Debug + Sync + Send {
    type Input;
    type Output;
    type Error = Box<dyn std::error::Error>;

    /// The name of this puzzle
    fn name(&self) -> String;

    /// The cases this puzzle has, including examples and solutions
    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>>;

    /// Run the puzzle infallibly. This might panic if the puzzle is not
    /// infallible
    fn run_puzzle(_input: Self::Input) -> Self::Output {
        unimplemented!();
    }

    /// Run the puzzle.
    ///
    /// # Errors
    /// Returns an error if the puzzle could not be solved
    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(Self::run_puzzle(input))
    }
}

impl<T: PuzzleRunner> Puzzle for T {
    fn name(&self) -> String {
        PuzzleRunner::name(self)
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        PuzzleRunner::cases(self)
    }
}

pub enum ExpectedValue<T> {
    Exact(T),
    None,
    Predicate(fn(&T) -> bool),
}

impl<T: std::fmt::Debug> std::fmt::Debug for ExpectedValue<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Exact(v) => write!(fmt, "ExpectedValue::Exact({:?})", v)?,
            Self::None => write!(fmt, "ExpectedValue::None")?,
            Self::Predicate(_) => write!(fmt, "ExpectedValue::Predicate(<>)")?,
        };
        Ok(())
    }
}

impl<T, U> From<U> for ExpectedValue<T>
where
    U: Into<Option<T>>,
{
    fn from(v: U) -> Self {
        let opt: Option<T> = v.into();
        match opt {
            Some(v) => Self::Exact(v),
            None => Self::None,
        }
    }
}

#[derive(Debug)]
pub struct GenericPuzzleCase<'a, T, I, O> {
    pub name: String,
    pub input: I,
    pub expected: ExpectedValue<O>,
    pub phantom: PhantomData<&'a T>,
}

pub struct PuzzleResult {
    pub status: PuzzleResultStatus,
    pub description: String,
    pub duration: Duration,
}

pub enum PuzzleResultStatus {
    Match,
    Fail,
    Unknown,
    Error,
}

impl<'a, T, I, O, E> PuzzleCase for GenericPuzzleCase<'a, T, I, O>
where
    T: PuzzleRunner<Input = I, Output = O, Error = E>,
    O: PartialEq + std::fmt::Debug + Sync + Send,
    I: Clone + std::fmt::Debug + Sync + Send,
    E: Display,
{
    fn name(&self) -> String {
        self.name.clone()
    }

    fn run(&self) -> PuzzleResult {
        let start = Instant::now();
        let actual_result = T::try_run_puzzle(self.input.clone());
        let duration = start.elapsed();

        if let Err(err) = actual_result {
            return PuzzleResult {
                status: PuzzleResultStatus::Error,
                duration,
                description: format!("Error: {}", err),
            };
        }
        let actual = actual_result.ok().unwrap();

        match self.expected {
            ExpectedValue::Exact(ref expected) => {
                if actual == *expected {
                    PuzzleResult {
                        status: PuzzleResultStatus::Match,
                        duration,
                        description: format!("{:?} is correct", actual),
                    }
                } else {
                    PuzzleResult {
                        status: PuzzleResultStatus::Fail,
                        duration,
                        description: format!("expected {:?} got {:?}", expected, actual),
                    }
                }
            }
            ExpectedValue::Predicate(predicate) => {
                if predicate(&actual) {
                    PuzzleResult {
                        status: PuzzleResultStatus::Unknown,
                        duration,
                        description: format!("{:?} matches predicate", actual),
                    }
                } else {
                    PuzzleResult {
                        status: PuzzleResultStatus::Fail,
                        duration,
                        description: format!("{:?} does not match predicate", actual),
                    }
                }
            }
            ExpectedValue::None => PuzzleResult {
                status: PuzzleResultStatus::Unknown,
                duration,
                description: format!("{:?}", actual),
            },
        }
    }
}

impl<'a, T, I, O, E> GenericPuzzleCase<'a, T, I, O>
where
    T: PuzzleRunner<Input = I, Output = O, Error = E>,
    O: PartialEq + 'a + std::fmt::Debug + Sync + Send,
    I: Clone + 'a + std::fmt::Debug + Sync + Send,
    E: Display,
{
    #[must_use]
    pub fn build_set() -> CaseSetBuilder<'a, T, I, O> {
        CaseSetBuilder::new()
    }
}

pub struct CaseSetBuilder<'a, T, I, O, E = Box<dyn std::error::Error>> {
    cases: Vec<GenericPuzzleCase<'a, T, I, O>>,
    try_transform: Option<Box<dyn Fn(&str) -> Result<I, E>>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, I, O, ERunner, ETransform> CaseSetBuilder<'a, T, I, O, ETransform>
where
    T: PuzzleRunner<Input = I, Output = O, Error = ERunner>,
    O: PartialEq + 'a + std::fmt::Debug + Sync + Send,
    I: Clone + 'a + std::fmt::Debug + Sync + Send,
    ERunner: Display,
{
    fn new() -> Self {
        Self {
            cases: vec![],
            try_transform: None,
            phantom: PhantomData::<&T>,
        }
    }

    pub fn add_transform<F>(mut self, transform: F) -> Self
    where
        F: Fn(&str) -> I + 'static,
    {
        self.try_transform = Some(Box::new(move |input| Ok(transform(input))));
        self
    }

    pub fn add_try_transform<F>(mut self, transform: F) -> Self
    where
        F: Fn(&str) -> Result<I, ETransform> + 'static,
    {
        self.try_transform = Some(Box::new(transform));
        self
    }

    pub fn case<S, I_, O_>(mut self, name: S, input: I_, expected: O_) -> Self
    where
        S: Into<String>,
        I_: Into<I>,
        O_: Into<ExpectedValue<O>>,
    {
        self.cases.push(GenericPuzzleCase {
            name: name.into(),
            input: input.into(),
            expected: expected.into(),
            phantom: self.phantom,
        });
        self
    }

    pub fn transformed_case<S, O_>(self, name: S, raw_input: &str, expected: O_) -> Result<Self, ETransform>
    where
        S: Into<String>,
        O_: Into<ExpectedValue<O>>,
    {
        match self.try_transform {
            Some(ref try_transform) => {
                let transformed_input = try_transform(raw_input)?;
                Ok(self.case(name, transformed_input, expected))
            }
            None => panic!("Must call `add_transform` before transformed_case"),
        }
    }

    #[must_use]
    pub fn collect(self) -> Vec<Box<dyn PuzzleCase + 'a>> {
        self.cases
            .into_iter()
            .map(|case| Box::new(case) as Box<dyn PuzzleCase + 'a>)
            .collect()
    }
}
