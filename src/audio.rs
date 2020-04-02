          pub trait MusicPlayer {
      fn load(&mut self, file: &str);
          fn seek(&mut self, val: f64);
              fn get_pos(&mut self) -> f64;
                  fn set_play(&mut self, val: bool);
                      fn set_loop(&mut self, val: bool);
                          fn set_speed(&mut self, val: f32);
                              fn set_volume(&mut self, val: f32);
                                  fn set_fromto(&mut self, from: f64, to: f64);
}
