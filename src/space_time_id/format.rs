use core::fmt;

use crate::space_time_id::SpaceTimeId;

impl fmt::Display for SpaceTimeId {
    ///暫定的に全てを範囲記法にする
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}:{}/{}:{}/{}:{}_{}/{}:{}",
            self.z,
            self.f[0],
            self.f[1],
            self.x[0],
            self.x[1],
            self.y[0],
            self.y[1],
            self.i,
            self.t[0],
            self.t[1],
        )
    }
}
