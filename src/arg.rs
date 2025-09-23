#![doc(hidden)]

use crate::Args;
use crate::Error;
use crate::Result;
use crate::lexer::Lexer;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::str::FromStr;

pub struct NoValue;

pub trait Value: Sized {
    fn parse(value: String) -> Result<Self>;
}

impl<T: FromStr<Err: Into<Box<dyn std::error::Error>>>> Value for T {
    fn parse(value: String) -> Result<Self> {
        match value.parse::<Self>() {
            Ok(value) => Ok(value),
            Err(error) => Err(Error::ParsingFailed {
                value,
                error: error.into(),
            }),
        }
    }
}

pub trait Command: Sized {
    fn normalize(command: &str) -> Result<&'static str>;
    fn from(command: &str, args: Args) -> Result<Self>;
}

#[inline]
pub fn option(names: &'static [&'static str]) -> ArgOption<NoValue, false> {
    ArgOption {
        names,
        conflicts: &[],
        used_name: None,
        on_set: None,
        value: vec![],
    }
}

#[inline]
pub fn non_option(name: &'static str) -> ArgNonOption<NoValue, false, false> {
    ArgNonOption {
        name,
        conflicts: &[],
        grants: 0,
        value: vec![],
    }
}

pub struct ArgOption<T, const VARIADIC: bool> {
    names: &'static [&'static str],
    conflicts: &'static [&'static str],
    used_name: Option<&'static str>,
    on_set: Option<Error>,
    value: Vec<T>,
}

pub struct ArgOptionAction<'a, T: Fn() -> Error> {
    names: &'static [&'static str],
    action: T,
    marker: std::marker::PhantomData<&'a T>,
}

impl ArgOption<NoValue, false> {
    #[inline]
    pub fn value<T: Value>(self) -> ArgOption<T, false> {
        ArgOption {
            names: self.names,
            conflicts: self.conflicts,
            used_name: self.used_name,
            on_set: self.on_set,
            value: vec![],
        }
    }

    #[inline]
    pub fn variadic(self) -> ArgOption<NoValue, true> {
        ArgOption {
            names: self.names,
            conflicts: self.conflicts,
            used_name: self.used_name,
            on_set: self.on_set,
            value: self.value,
        }
    }

    #[inline]
    pub fn version<'a>(
        self,
        name: &'a str,
        version: &'a str,
    ) -> ArgOptionAction<'a, impl Fn() -> Error> {
        ArgOptionAction {
            names: self.names,
            action: move || Error::Version {
                message: format!("{name} {version}"),
            },
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn help<'a>(self, message: &'a [&'a str]) -> ArgOptionAction<'a, impl Fn() -> Error> {
        ArgOptionAction {
            names: self.names,
            action: move || Error::Help {
                message: message.concat(),
            },
            marker: PhantomData,
        }
    }
}

impl<T: Value> ArgOption<T, false> {
    #[inline]
    pub fn variadic(self) -> ArgOption<T, true> {
        ArgOption {
            names: self.names,
            conflicts: self.conflicts,
            used_name: self.used_name,
            on_set: self.on_set,
            value: self.value,
        }
    }
}

impl<T, const VARIADIC: bool> ArgOption<T, VARIADIC> {
    #[inline]
    pub fn conflicts(mut self, conflicts: &'static [&'static str]) -> Self {
        self.conflicts = conflicts;
        self
    }

    fn try_match(&self, option: &str) -> Option<&'static str> {
        self.names
            .iter()
            .find_map(|opt| if *opt == option { Some(*opt) } else { None })
    }
}

impl<const VARIADIC: bool> ArgOption<NoValue, VARIADIC> {
    #[inline]
    pub fn as_setter(&mut self) -> &mut dyn ArgOptionSetter {
        self as &mut dyn ArgOptionSetter
    }
}

impl<T: Value, const VARIADIC: bool> ArgOption<T, VARIADIC> {
    #[inline]
    pub fn as_setter(&mut self) -> &mut dyn ArgOptionSetter {
        self as &mut dyn ArgOptionSetter
    }
}

impl<T: Fn() -> Error> ArgOptionAction<'_, T> {
    #[inline]
    pub fn as_setter(&mut self) -> &mut dyn ArgOptionSetter {
        self as &mut dyn ArgOptionSetter
    }

    fn try_match(&self, option: &str) -> Option<&'static str> {
        self.names
            .iter()
            .find_map(|opt| if *opt == option { Some(*opt) } else { None })
    }
}

impl<T: Fn() -> Error> ArgOptionAction<'_, T> {
    fn set(&mut self) -> Result<()> {
        Err((self.action)())
    }
}

impl<const VARIADIC: bool> ArgOption<NoValue, VARIADIC> {
    fn set(&mut self, option: &'static str) -> Result<()> {
        self.used_name = Some(option);
        self.value.push(NoValue);
        match self.on_set.take() {
            Some(error) => Err(error),
            _ => Ok(()),
        }
    }
}

impl<T: Value, const VARIADIC: bool> ArgOption<T, VARIADIC> {
    fn set(&mut self, option: &'static str, arg: String) -> Result<()> {
        self.used_name = Some(option);
        self.value.push(T::parse(arg)?);
        Ok(())
    }
}

impl ArgOption<NoValue, false> {
    #[inline]
    pub fn into(self) -> bool {
        !self.value.is_empty()
    }
}

impl ArgOption<NoValue, true> {
    #[inline]
    pub fn into(self) -> usize {
        self.value.len()
    }
}

impl<T: Value> ArgOption<T, false> {
    #[inline]
    pub fn into(mut self) -> Option<T> {
        self.value.pop()
    }
}

impl<T: Value> ArgOption<T, true> {
    #[inline]
    pub fn into(self) -> Vec<T> {
        self.value
    }
}

pub struct ArgNonOption<T, const OPTIONAL: bool, const VARIADIC: bool> {
    name: &'static str,
    conflicts: &'static [&'static str],
    grants: usize,
    value: Vec<T>,
}

impl ArgNonOption<NoValue, false, false> {
    #[inline]
    pub fn value<T: Value>(self) -> ArgNonOption<T, false, false> {
        ArgNonOption {
            name: self.name,
            conflicts: self.conflicts,
            grants: self.grants,
            value: vec![],
        }
    }

    #[inline]
    pub fn command<T: Command>(self) -> ArgNonOptionCommand<T, false> {
        ArgNonOptionCommand {
            name: self.name,
            conflicts: self.conflicts,
            grants: self.grants,
            value: vec![],
            marker: PhantomData,
        }
    }
}

impl<T: Value> ArgNonOption<T, false, false> {
    #[inline]
    pub fn optional(self) -> ArgNonOption<T, true, false> {
        ArgNonOption {
            name: self.name,
            conflicts: self.conflicts,
            grants: self.grants,
            value: self.value,
        }
    }
}

impl<T: Value, const OPTIONAL: bool> ArgNonOption<T, OPTIONAL, false> {
    #[inline]
    pub fn variadic(self) -> ArgNonOption<T, OPTIONAL, true> {
        ArgNonOption {
            name: self.name,
            conflicts: self.conflicts,
            grants: self.grants,
            value: self.value,
        }
    }
}

impl<T: Value, const OPTIONAL: bool, const VARIADIC: bool> ArgNonOption<T, OPTIONAL, VARIADIC> {
    #[inline]
    pub fn conflicts(mut self, conflicts: &'static [&'static str]) -> Self {
        self.conflicts = conflicts;
        self
    }

    #[inline]
    pub fn as_setter(&mut self) -> &mut dyn ArgNonOptionSetter {
        self as &mut dyn ArgNonOptionSetter
    }
}

impl<T: Value> ArgNonOption<T, false, false> {
    #[inline]
    pub fn into(mut self) -> T {
        self.value.pop().unwrap()
    }
}

impl<T: Value> ArgNonOption<T, true, false> {
    #[inline]
    pub fn into(mut self) -> Option<T> {
        self.value.pop()
    }
}

impl<T: Value> ArgNonOption<T, false, true> {
    #[inline]
    pub fn into(self) -> Vec<T> {
        self.value
    }
}

impl<T: Value> ArgNonOption<T, true, true> {
    #[inline]
    pub fn into(self) -> Vec<T> {
        self.value
    }
}

pub struct ArgNonOptionCommand<T, const OPTIONAL: bool> {
    name: &'static str,
    conflicts: &'static [&'static str],
    grants: usize,
    value: Vec<String>,
    marker: PhantomData<T>,
}

impl<T: Command> ArgNonOptionCommand<T, false> {
    #[inline]
    pub fn optional(self) -> ArgNonOptionCommand<T, true> {
        ArgNonOptionCommand {
            name: self.name,
            conflicts: self.conflicts,
            grants: self.grants,
            value: self.value,
            marker: self.marker,
        }
    }
}

impl<T: Command, const OPTIONAL: bool> ArgNonOptionCommand<T, OPTIONAL> {
    #[inline]
    pub fn conflicts(mut self, conflicts: &'static [&'static str]) -> Self {
        self.conflicts = conflicts;
        self
    }

    #[inline]
    pub fn as_setter(&mut self) -> &mut dyn ArgNonOptionSetter {
        self as &mut dyn ArgNonOptionSetter
    }
}

impl<T: Command> ArgNonOptionCommand<T, false> {
    pub fn into(self, bin_name: &str) -> Result<T> {
        let mut args = Args::from_vec(self.value);
        let arg0 = args.peek().unwrap();
        let command = T::normalize(arg0)?;
        args.set_bin_name(format!("{bin_name} {command}"));
        T::from(command, args)
    }
}

impl<T: Command> ArgNonOptionCommand<T, true> {
    pub fn into(self, bin_name: &str) -> Result<Option<T>> {
        let mut args = Args::from_vec(self.value);
        let Some(arg0) = args.peek() else {
            return Ok(None);
        };
        let command = T::normalize(arg0)?;
        args.set_bin_name(format!("{bin_name} {command}"));
        Ok(Some(T::from(command, args)?))
    }
}

pub trait ArgOptionSetter {
    fn names(&self) -> &'_ [&'_ str];
    fn used_name(&self) -> &'_ str;
    fn conflicts(&self) -> &'_ [&'_ str];
    fn is_set(&self) -> bool;
    fn try_match(&self, option: &str) -> Option<&'static str>;
    fn takes_value(&self) -> bool;
    fn set(&mut self, option: &'static str, value: Option<String>) -> Result<()>;
}

impl<const VARIADIC: bool> ArgOptionSetter for ArgOption<NoValue, VARIADIC> {
    fn names(&self) -> &'_ [&'_ str] {
        self.names
    }

    fn used_name(&self) -> &'_ str {
        self.used_name.unwrap()
    }

    fn is_set(&self) -> bool {
        !self.value.is_empty()
    }

    fn conflicts(&self) -> &'_ [&'_ str] {
        self.conflicts
    }

    fn try_match(&self, option: &str) -> Option<&'static str> {
        self.try_match(option)
    }

    fn takes_value(&self) -> bool {
        false
    }

    fn set(&mut self, option: &'static str, _value: Option<String>) -> Result<()> {
        self.set(option)
    }
}

impl<T: Value, const VARIADIC: bool> ArgOptionSetter for ArgOption<T, VARIADIC> {
    fn names(&self) -> &'_ [&'_ str] {
        self.names
    }

    fn used_name(&self) -> &'_ str {
        self.used_name.unwrap()
    }

    fn is_set(&self) -> bool {
        !self.value.is_empty()
    }

    fn conflicts(&self) -> &'_ [&'_ str] {
        self.conflicts
    }

    fn try_match(&self, option: &str) -> Option<&'static str> {
        self.try_match(option)
    }

    fn takes_value(&self) -> bool {
        true
    }

    fn set(&mut self, option: &'static str, value: Option<String>) -> Result<()> {
        self.set(option, value.unwrap())
    }
}

impl<T: Fn() -> Error> ArgOptionSetter for ArgOptionAction<'_, T> {
    fn names(&self) -> &'_ [&'_ str] {
        self.names
    }

    fn used_name(&self) -> &'_ str {
        unreachable!()
    }

    fn is_set(&self) -> bool {
        false
    }

    fn conflicts(&self) -> &'_ [&'_ str] {
        &[]
    }

    fn try_match(&self, option: &str) -> Option<&'static str> {
        self.try_match(option)
    }

    fn takes_value(&self) -> bool {
        false
    }

    fn set(&mut self, _option: &'static str, _value: Option<String>) -> Result<()> {
        self.set()
    }
}

pub trait ArgNonOptionSetter {
    fn name(&self) -> &'_ str;
    fn is_optional(&self) -> bool;
    fn is_variadic(&self) -> bool;
    fn is_set(&self) -> bool;
    fn conflicts(&self) -> &'_ [&'_ str];
    fn grant(&mut self, num_args: usize);
    fn grants(&self) -> usize;
    fn set(&mut self, arg: String) -> Result<()>;
}

impl<T: Value, const OPTIONAL: bool, const VARIADIC: bool> ArgNonOptionSetter
    for ArgNonOption<T, OPTIONAL, VARIADIC>
{
    fn name(&self) -> &'_ str {
        self.name
    }

    fn is_optional(&self) -> bool {
        OPTIONAL
    }

    fn is_variadic(&self) -> bool {
        VARIADIC
    }

    fn is_set(&self) -> bool {
        !self.value.is_empty()
    }

    fn conflicts(&self) -> &'_ [&'_ str] {
        self.conflicts
    }

    fn grant(&mut self, num_args: usize) {
        self.grants += num_args;
    }

    fn grants(&self) -> usize {
        self.grants
    }

    fn set(&mut self, arg: String) -> Result<()> {
        self.value.push(T::parse(arg)?);
        Ok(())
    }
}

impl<T: Command, const OPTIONAL: bool> ArgNonOptionSetter for ArgNonOptionCommand<T, OPTIONAL> {
    fn name(&self) -> &'_ str {
        self.name
    }

    fn is_optional(&self) -> bool {
        OPTIONAL
    }

    fn is_variadic(&self) -> bool {
        true
    }

    fn is_set(&self) -> bool {
        !self.value.is_empty()
    }

    fn conflicts(&self) -> &'_ [&'_ str] {
        self.conflicts
    }

    fn grant(&mut self, num_args: usize) {
        self.grants += num_args;
    }

    fn grants(&self) -> usize {
        self.grants
    }

    fn set(&mut self, arg: String) -> Result<()> {
        self.value.push(arg);
        Ok(())
    }
}

pub fn parse(
    args: Args,
    options: &mut [&mut dyn ArgOptionSetter],
    non_options: &mut [&mut dyn ArgNonOptionSetter],
) -> Result<()> {
    let mut l = Lexer::new(args);
    set_options(&mut l, options)?;
    set_non_options(&mut l, non_options)?;
    check_conflicts_and_choices(options, non_options)?;
    Ok(())
}

fn set_options(lexer: &mut Lexer, setters: &mut [&mut dyn ArgOptionSetter]) -> Result<()> {
    'next: while let Some(option) = lexer.next_option()? {
        let Some((setter, option)) = setters
            .iter_mut()
            .find_map(|setter| setter.try_match(option).map(|option| (setter, option)))
        else {
            return Err(Error::InvalidOption {
                option: option.to_string(),
            });
        };

        let value = if setter.takes_value() {
            Some(lexer.next_value()?)
        } else {
            None
        };

        setter.set(option, value)?;
        continue 'next;
    }

    Ok(())
}

fn set_non_options(lexer: &mut Lexer, setters: &mut [&mut dyn ArgNonOptionSetter]) -> Result<()> {
    let args = lexer.non_options()?;
    let mut num_args = args.0.len();

    for required in setters
        .iter_mut()
        .filter(|setter| !setter.is_optional())
        .take(num_args)
    {
        required.grant(1);
        num_args -= 1;
    }

    for optional in setters
        .iter_mut()
        .filter(|setter| setter.is_optional())
        .take(num_args)
    {
        optional.grant(1);
        num_args -= 1;
    }

    if num_args > 0
        && let Some(variadic) = setters.iter_mut().find(|setter| setter.is_variadic())
    {
        variadic.grant(num_args);
    }

    for setter in setters.iter_mut() {
        for arg in args.0.drain(..setter.grants()) {
            setter.set(arg)?;
        }
    }

    if let Some(required) = setters
        .iter()
        .find(|arg| !arg.is_optional() && !arg.is_set())
    {
        return Err(Error::MissingArgument {
            arg: required.name().to_string(),
        });
    }

    match args.0.pop_front() {
        None => Ok(()),
        Some(arg) => Err(Error::InvalidArgument { arg }),
    }
}

fn check_conflicts_and_choices(
    options: &[&mut dyn ArgOptionSetter],
    non_options: &[&mut dyn ArgNonOptionSetter],
) -> Result<()> {
    let mut conflicts = HashMap::new();

    for arg1 in options.iter().filter(|arg| arg.is_set()) {
        for conflict in arg1.conflicts() {
            let name1 = arg1.used_name();
            if let Some(name0) = conflicts.insert(conflict, name1) {
                return Err(Error::ConflictingArguments {
                    arg0: name0.to_string(),
                    arg1: name1.to_string(),
                });
            }
        }
    }

    for arg1 in non_options.iter().filter(|arg| arg.is_set()) {
        for conflict in arg1.conflicts() {
            let name1 = arg1.name();
            if let Some(name0) = conflicts.insert(conflict, name1) {
                return Err(Error::ConflictingArguments {
                    arg0: name0.to_string(),
                    arg1: name1.to_string(),
                });
            }
        }
    }

    let mut choices = HashMap::new();
    const QUESTION: char = '?';
    const DASHDASH: &str = "--";

    for arg in options {
        for conflict in arg
            .conflicts()
            .iter()
            .filter(|conflict| conflict.starts_with(QUESTION))
        {
            let names = arg.names();
            let primary_name = match names.iter().find(|name| name.starts_with(DASHDASH)) {
                Some(first_long) => first_long,
                _ => names[0],
            };
            choices.entry(conflict).or_insert(vec![]).push(primary_name);
        }
    }

    for arg in non_options {
        for conflict in arg
            .conflicts()
            .iter()
            .filter(|conflict| conflict.starts_with(QUESTION))
        {
            choices.entry(conflict).or_insert(vec![]).push(arg.name());
        }
    }

    for (choice, alternatives) in choices {
        if !conflicts.contains_key(choice) {
            let alternatives = alternatives
                .iter()
                .map(|alternative| alternative.to_string())
                .collect::<Vec<_>>();
            return Err(Error::MissingChoice { alternatives });
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::Args;
    use crate::Error;
    use crate::arg::ArgNonOptionSetter;
    use crate::arg::non_option;
    use crate::arg::option;
    use crate::lexer::Lexer;

    #[test]
    fn set_non_option_0() {
        let mut l = Lexer::new(Args::from(["0"]));
        let result = super::set_non_options(&mut l, &mut []);

        let error = result.err().unwrap();
        assert!(matches!(error, Error::InvalidArgument { arg }
            if arg == "0"
        ));
    }

    #[test]
    fn set_non_option_1() {
        let mut a = non_option("<a>").value::<u64>();
        let mut b = non_option("<b>").value::<u64>();

        let mut l = Lexer::new(Args::from(["0"]));
        let result = super::set_non_options(&mut l, &mut [a.as_setter(), b.as_setter()]);

        let error = result.err().unwrap();
        assert!(matches!(error, Error::MissingArgument { arg }
            if arg == "<b>"
        ));
    }

    #[test]
    fn set_non_option_2() {
        let mut a = non_option("[a]").value::<u64>().optional();
        let mut b = non_option("<b>").value::<u64>();
        let mut c = non_option("[c...]").value::<u64>().optional().variadic();
        let mut d = non_option("<d>").value::<u64>();

        let mut l = Lexer::new(Args::from(["0", "1"]));
        let result = super::set_non_options(
            &mut l,
            &mut [a.as_setter(), b.as_setter(), c.as_setter(), d.as_setter()],
        );

        assert!(result.is_ok());
        assert!(a.into().is_none());
        assert!(b.into() == 0);
        assert!(c.into().is_empty());
        assert!(d.into() == 1);
    }

    #[test]
    fn set_non_option_3() {
        let mut a = non_option("<a>").value::<u64>().optional();
        let mut b = non_option("<b>").value::<u64>();
        let mut c = non_option("[c...]").value::<u64>().optional().variadic();
        let mut d = non_option("<d>").value::<u64>();

        let mut l = Lexer::new(Args::from(["0", "1", "2"]));
        let result = super::set_non_options(
            &mut l,
            &mut [a.as_setter(), b.as_setter(), c.as_setter(), d.as_setter()],
        );

        assert!(result.is_ok());
        assert!(a.into() == Some(0));
        assert!(b.into() == 1);
        assert!(c.into().is_empty());
        assert!(d.into() == 2);
    }

    #[test]
    fn set_non_option_4() {
        let mut a = non_option("[a]").value::<u64>().optional();
        let mut b = non_option("<b>").value::<u64>();
        let mut c = non_option("[c...]").value::<u64>().optional().variadic();
        let mut d = non_option("<d>").value::<u64>();

        let mut l = Lexer::new(Args::from(["0", "1", "2", "3", "4", "5"]));
        let result = super::set_non_options(
            &mut l,
            &mut [a.as_setter(), b.as_setter(), c.as_setter(), d.as_setter()],
        );

        assert!(result.is_ok());
        assert!(a.into() == Some(0));
        assert!(b.into() == 1);
        assert!(c.into() == vec![2, 3, 4]);
        assert!(d.into() == 5);
    }

    #[test]
    fn check_conflict_none_set() {
        let mut a = option(&["-a"]).value::<u64>().conflicts(&["0"]);
        let mut b = option(&["-b"]).value::<u64>().conflicts(&["0"]);
        let mut c = non_option("<c>").value::<u64>().conflicts(&["0"]);
        let mut d = non_option("<d>").value::<u64>().conflicts(&["0"]);

        let result = super::check_conflicts_and_choices(
            &[a.as_setter(), b.as_setter()],
            &[c.as_setter(), d.as_setter()],
        );

        assert!(result.is_ok());
    }

    #[test]
    fn check_conflict_one_set() {
        let mut a = option(&["-a"]).value::<u64>().conflicts(&["0"]);
        let mut b = option(&["-b"]).value::<u64>().conflicts(&["0"]);
        let mut c = non_option("<c>").value::<u64>().conflicts(&["0"]);
        let mut d = non_option("<d>").value::<u64>().conflicts(&["0"]);

        b.set("-b", "47".into()).unwrap();

        let result = super::check_conflicts_and_choices(
            &[a.as_setter(), b.as_setter()],
            &[c.as_setter(), d.as_setter()],
        );

        assert!(result.is_ok());
    }

    #[test]
    fn check_conflict_two_set() {
        let mut a = option(&["-a"]).value::<u64>().conflicts(&["0"]);
        let mut b = option(&["-b"]).value::<u64>().conflicts(&["0"]);
        let mut c = non_option("<c>").value::<u64>().conflicts(&["0"]);
        let mut d = non_option("<d>").value::<u64>().conflicts(&["0"]);

        b.set("-b", "47".into()).unwrap();
        d.set("47".into()).unwrap();

        let result = super::check_conflicts_and_choices(
            &[a.as_setter(), b.as_setter()],
            &[c.as_setter(), d.as_setter()],
        );

        let error = result.err().unwrap();
        assert!(matches!(error, Error::ConflictingArguments { arg0, arg1 }
            if arg0 == "-b" && arg1 == "<d>"
        ));
    }

    #[test]
    fn check_conflict_all_set() {
        let mut a = option(&["-a"]).value::<u64>().conflicts(&["0"]);
        let mut b = option(&["-b"]).value::<u64>().conflicts(&["0"]);
        let mut c = non_option("<c>").value::<u64>().conflicts(&["0"]);
        let mut d = non_option("<d>").value::<u64>().conflicts(&["0"]);

        a.set("-a", "47".into()).unwrap();
        b.set("-b", "47".into()).unwrap();
        c.set("47".into()).unwrap();
        d.set("47".into()).unwrap();

        let result = super::check_conflicts_and_choices(
            &[a.as_setter(), b.as_setter()],
            &[c.as_setter(), d.as_setter()],
        );

        let error = result.err().unwrap();
        assert!(matches!(error, Error::ConflictingArguments { arg0, arg1 }
            if arg0 == "-a" && arg1 == "-b"
        ));
    }

    #[test]
    fn check_choice() {
        let mut a = option(&["-a"]).value::<u64>().conflicts(&["?0"]);
        let mut b = option(&["-b"]).value::<u64>().conflicts(&["!0"]);
        let mut c = non_option("<c>").value::<u64>().conflicts(&["?0"]);
        let mut d = non_option("<d>").value::<u64>().conflicts(&["!0"]);

        // FIXME
        //a.set("-a", "47".into()).unwrap();
        //b.set("-b", "47".into()).unwrap();
        //c.set("47".into()).unwrap();
        //d.set("47".into()).unwrap();

        let result = super::check_conflicts_and_choices(
            &[a.as_setter(), b.as_setter()],
            &[c.as_setter(), d.as_setter()],
        );

        let error = result.err().unwrap();
        assert!(matches!(error, Error::MissingChoice { alternatives }
            if alternatives[0] == "-a" && alternatives[1] == "<c>"
        ));
    }
}
