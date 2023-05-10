use crate::{SNssai, Snssai};

impl From<Snssai> for SNssai {
    fn from(x: Snssai) -> Self {
        SNssai {
            sst: x.sst,
            sd: x.sd,
        }
    }
}

impl From<SNssai> for Snssai {
    fn from(x: SNssai) -> Self {
        Snssai {
            sst: x.sst,
            sd: x.sd,
        }
    }
}
