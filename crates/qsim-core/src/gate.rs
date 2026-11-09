#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Gate {
    H { target: usize },
    X { target: usize },
    Cx { control: usize, target: usize },
}

impl Gate {
    pub(crate) fn qubits(self) -> [Option<usize>; 2] {
        match self {
            Self::H { target } | Self::X { target } => [Some(target), None],
            Self::Cx { control, target } => [Some(control), Some(target)],
        }
    }
}
