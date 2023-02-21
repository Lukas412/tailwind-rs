#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PseudoClass {
    Hover,
    Focus,
    FocusWithin,
    FocusVisible,
    Active,
    Visited,
    Target,
    First,
    Last,
    Only,
    Odd,
    Even,
    FirstOfType,
    LastOfType,
    OnlyOfType,
    Empty,
    Disabled,
    Enabled,
    Checked,
    Indeterminate,
    Default,
    Required,
    Valid,
    Invalid,
    InRange,
    OutOfRange,
    PlaceholderShown,
    Autofill,
    ReadOnly,
    Open,
}

impl PseudoClass {
    pub fn new(value: &str) -> Option<Self> {
        let pc = match value {
            "hover" => Self::Hover,
            "focus" => Self::Focus,
            "focus-within" => Self::FocusWithin,
            "focus-visible" => Self::FocusVisible,
            "active" => Self::Active,
            "visited" => Self::Visited,
            "target" => Self::Target,
            "first" => Self::First,
            "last" => Self::Last,
            "only" => Self::Only,
            "odd" => Self::Odd,
            "even" => Self::Even,
            "first-of-type" => Self::FirstOfType,
            "last-of-type" => Self::LastOfType,
            "only-of-type" => Self::OnlyOfType,
            "empty" => Self::Empty,
            "disabled" => Self::Disabled,
            "enabled" => Self::Enabled,
            "checked" => Self::Checked,
            "indeterminate" => Self::Indeterminate,
            "default" => Self::Default,
            "required" => Self::Required,
            "valid" => Self::Valid,
            "invalid" => Self::Invalid,
            "in-range" => Self::InRange,
            "out-of-range" => Self::OutOfRange,
            "placeholder-shown" => Self::PlaceholderShown,
            "autofill" => Self::Autofill,
            "readonly" => Self::ReadOnly,
            "open" => Self::Open,
            _ => return None,
        };

        Some(pc)
    }

    pub fn to_static_str(self) -> &'static str {
        match self {
            Self::Hover => "hover",
            Self::Focus => "focus",
            Self::FocusWithin => "focus-within",
            Self::FocusVisible => "focus-visible",
            Self::Active => "active",
            Self::Visited => "visited",
            Self::Target => "target",
            Self::First => "first-child",
            Self::Last => "last-child",
            Self::Only => "only-child",
            Self::Odd => "nth-child(odd)",
            Self::Even => "nth-child(even)",
            Self::FirstOfType => "first-of-type",
            Self::LastOfType => "last-of-type",
            Self::OnlyOfType => "only-of-type",
            Self::Empty => "empty",
            Self::Disabled => "disabled",
            Self::Enabled => "enabled",
            Self::Checked => "checked",
            Self::Indeterminate => "indeterminate",
            Self::Default => "default",
            Self::Required => "required",
            Self::Valid => "valid",
            Self::Invalid => "invalid",
            Self::InRange => "in-range",
            Self::OutOfRange => "out-of-range",
            Self::PlaceholderShown => "placeholder-shown",
            Self::Autofill => "autofill",
            Self::ReadOnly => "readonly",
            Self::Open => "open",
        }
    }
}
