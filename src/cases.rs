use std::marker::PhantomData;

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
    fn cases(&self) -> Vec<Box<dyn PuzzleCase>>;
}

/// A function to run a specific puzzle's code
///
/// In contrast to `Puzzle`, this trait contains the specific types for the
/// puzzle.
pub trait PuzzleRunner: std::fmt::Debug + Sync + Send {
    type Input;
    type Output;

    fn name(&self) -> String;
    fn cases(&self) -> Vec<Box<dyn PuzzleCase>>;
    fn run_puzzle(input: Self::Input) -> Self::Output;
}

impl<T: PuzzleRunner> Puzzle for T {
    fn name(&self) -> String {
        PuzzleRunner::name(self)
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        PuzzleRunner::cases(self)
    }
}

#[derive(Debug)]
pub struct GenericPuzzleCase<'a, T, I, O> {
    pub name: String,
    pub input: I,
    pub expected: Option<O>,
    pub phantom: PhantomData<&'a T>,
}

pub enum PuzzleResult {
    Match,
    Fail { description: String },
    Unknown { description: String },
}

impl<'a, T, I, O> PuzzleCase for GenericPuzzleCase<'a, T, I, O>
where
    T: PuzzleRunner<Input = I, Output = O>,
    O: PartialEq + std::fmt::Debug + Sync + Send,
    I: Clone + std::fmt::Debug + Sync + Send,
{
    fn name(&self) -> String {
        self.name.clone()
    }

    fn run(&self) -> PuzzleResult {
        let actual = T::run_puzzle(self.input.clone());
        if let Some(ref expected) = self.expected {
            if actual == *expected {
                PuzzleResult::Match
            } else {
                PuzzleResult::Fail {
                    description: format!("expected {:?} got {:?}", expected, actual),
                }
            }
        } else {
            PuzzleResult::Unknown {
                description: format!("{:?}", actual),
            }
        }
    }
}

impl<'a, T, I, O> GenericPuzzleCase<'a, T, I, O>
where
    T: PuzzleRunner<Input = I, Output = O>,
    O: PartialEq + 'a + std::fmt::Debug + Sync + Send,
    I: Clone + 'a + std::fmt::Debug + Sync + Send,
{
    pub fn build_set() -> CaseSetBuilder<'a, T, I, O> {
        CaseSetBuilder::new()
    }
}

pub struct CaseSetBuilder<'a, T, I, O> {
    cases: Vec<GenericPuzzleCase<'a, T, I, O>>,
    transform: Option<Box<dyn Fn(&str) -> I>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, I, O> CaseSetBuilder<'a, T, I, O>
where
    T: PuzzleRunner<Input = I, Output = O>,
    O: PartialEq + 'a + std::fmt::Debug + Sync + Send,
    I: Clone + 'a + std::fmt::Debug + Sync + Send,
{
    fn new() -> Self {
        Self {
            cases: vec![],
            transform: None,
            phantom: PhantomData::<&T>,
        }
    }

    pub fn add_transform<F>(mut self, transform: F) -> Self
    where
        F: Fn(&str) -> I,
        F: 'static,
    {
        self.transform = Some(Box::new(transform));
        self
    }

    pub fn case<S, I_, O_>(mut self, name: S, input: I_, expected: O_) -> Self
    where
        S: Into<String>,
        I_: Into<I>,
        O_: Into<Option<O>>,
    {
        self.cases.push(GenericPuzzleCase {
            name: name.into(),
            input: input.into(),
            expected: expected.into(),
            phantom: self.phantom,
        });
        self
    }

    pub fn transformed_case<S: Into<String>>(self, name: S, raw_input: &str, expected: O) -> Self {
        let transform = self
            .transform
            .deref()
            .expect("Must call add_transform before transformed_case");
        let transformed_input = transform(raw_input);
        self.case(name, transformed_input, expected)
    }

    pub fn collect(self) -> Vec<Box<dyn PuzzleCase + 'a>> {
        self.cases
            .into_iter()
            .map(|case| Box::new(case) as Box<dyn PuzzleCase + 'a>)
            .collect()
    }
}
