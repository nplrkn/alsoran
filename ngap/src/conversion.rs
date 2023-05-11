use crate::{ies::Snssai, Sd, Sst};

impl From<xxap::Snssai> for Snssai {
    fn from(x: xxap::Snssai) -> Self {
        Snssai {
            sst: Sst(vec![x.0]),
            sd: x.1.map(|array| Sd(array.to_vec())),
        }
    }
}

impl From<Snssai> for xxap::Snssai {
    fn from(x: Snssai) -> Self {
        xxap::Snssai(x.sst.0[0], x.sd.map(|x| [x.0[0], x.0[1], x.0[2]]))
    }
}
