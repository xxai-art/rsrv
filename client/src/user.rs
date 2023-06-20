const R_CLIENT_USER: &[u8] = &[4, 0];

impl crate::Client {
  pub fn user_id(&mut self) -> Option<u64> {
    match self._user_id {
      Some(id) => {
        if id == 0 {
          let bin = u64_bin(id);

          //zmax
          dbg!(id, "TODO get user_id");
          // self.user_id = Some(id);
          self._user_id = None;
          None
        } else {
          Some(id)
        }
      }
      None => None,
    }
  }
}
