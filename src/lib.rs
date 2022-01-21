//! # Hangeul ID
//!
//! `Hangeul ID` is a shorter unique id library using Hangeul
//!
use crate::hangeul::HangeulCharGroup;
use crate::hangeul::ksx1001::ksx1001_letters;

pub mod base;
pub mod hangeul;

/// Struct of HangeulId
pub struct HangeulId {
  pub char_group: HangeulCharGroup
}

impl HangeulId {
  /// Return HangeulId struct with KSX1001
  pub fn new() -> HangeulId {
    HangeulId {
      char_group: HangeulCharGroup::KSX1001
    }
  }

  /// Return HangeulId struct with character group
  ///
  /// # Arguments
  ///
  /// * `char_group` - HangeulCharGroup value
  ///
  /// # Examples
  /// ```
  /// use hangeul_id::HangeulId;
  /// use hangeul_id::hangeul::HangeulCharGroup;
  ///
  /// let hangeulid = HangeulId::new_with_char_group(HangeulCharGroup::KSX1001);
  /// ```
  pub fn new_with_char_group(char_group: HangeulCharGroup) -> HangeulId {
    HangeulId {
      char_group
    }
  }

  /// Return Hangeul id from integer
  ///
  /// # Arguments
  ///
  /// * `integer` - usize value
  ///
  /// # Examples
  ///
  /// ```
  /// use hangeul_id::HangeulId;
  ///
  /// let hangeulid = HangeulId::new();
  /// let id = hangeulid.int_to_hangeul_id(5529549);
  /// assert_eq!(id, "각간힝");
  /// ```
  pub fn int_to_hangeul_id(&self, integer: usize) -> String {
    match self.char_group {
      HangeulCharGroup::KSX1001 => ksx1001_letters(integer),
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn int_to_hangeul_id_default_test() {
    let hangeulid = HangeulId::new();
    let res = hangeulid.int_to_hangeul_id(5529549);
    assert_eq!(res, "각간힝".to_string());
  }

  #[test]
  fn int_to_hangeul_id_ksx1001_test() {
    let hangeulid = HangeulId::new_with_char_group(HangeulCharGroup::KSX1001);
    let res = hangeulid.int_to_hangeul_id(5529549);
    assert_eq!(res, "각간힝".to_string());
  }
}
