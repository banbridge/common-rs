// #[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
// pub struct LogId {
//     private: faststr::FastStr,
// }

// impl LogId {
//     pub fn fast_str(&self) -> faststr::FastStr {
//         self.private.clone()
//     }
// }

// impl ::std::ops::Deref for LogId {
//     type Target = str;
//     fn deref(&self) -> &Self::Target {
//         &self.private
//     }
// }

// impl<T> From<T> for LogId
// where
//     T: Into<faststr::FastStr>,
// {
//     fn from(s: T) -> Self {
//         Self { private: s.into() }
//     }
// }

pub type LogId = faststr::FastStr;
