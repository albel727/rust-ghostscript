#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct DeviceList(String);

impl DeviceList {
    pub fn new<T: AsRef<str>>(s: T) -> DeviceList {
        s.as_ref().split_whitespace().collect()
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'a, T: AsRef<str> + ?Sized + 'a> ::std::iter::FromIterator<&'a T> for DeviceList {
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        iter.into_iter().into()
    }
}

impl<'a, T: AsRef<str> + ?Sized + 'a> ::std::iter::Extend<&'a T> for DeviceList {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        let iter = iter.into_iter().flat_map(|j| j.as_ref().split_whitespace());
        for s in iter {
            if s.is_empty() {
                continue;
            }
            if !self.0.is_empty() {
                self.0.push(' ');
            }
            self.0.push_str(s)
        }
    }
}

impl<'a, T: AsRef<str> + ?Sized + 'a, I: IntoIterator<Item = &'a T>> From<I> for DeviceList {
    fn from(s: I) -> Self {
        let mut dl = DeviceList::default();
        dl.extend(s.into_iter());
        dl
    }
}

impl<'a> IntoIterator for &'a DeviceList {
    type Item = &'a str;
    type IntoIter = ::std::str::SplitWhitespace<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.split_whitespace()
    }
}
