use crate::pac::CAN;
use crate::pac::CAN_NODE0

const NODE_MAX_PRESCALER: u32 = 512;

const NODE_MAX_NTQ: u32 = 25;
const NODE_MIN_NTQ: u32 = 8;

const NODE_MIN_TSEG1: u32 = 3;
const NODE_MIN_TSEG2: u32 = 2;

const NODE_MAX_TSEG1: u32 = 15;
const NODE_MAX_TSEG2: u32 = 7;

pub enum Status {
    Success,
    Error,
    Busy,
    MoNotAcceptable,
    MoDisabled,
}

struct NodeNominalBitTimeCfg {
    frequency: u32,
    baudrate: u32,
    sample_point: u32,
    sjw: u32,
}

pub struct Can {
    pub regs: CAN,
}

impl Can {
    pub fn new(can: CAN) -> Self {
        Self { regs: can }
    }

    fn node_nominal_bit_time_configure_ex(&self, cfg: NodeNominalBitTimeCfg) -> Status {
        let status: Status = Status::Error;

        if (cfg.frequency % cfg.baudrate) == 0 {
            let mut prescaler: u32;
            let mut div8: u32;

            let fcan_div: u32 = cfg.frequency / cfg.baudrate;
            let mut ntq = NODE_MAX_NTQ;
            let mut tseg1: u32;
            let mut tseg2: u32;

            while ntq >= NODE_MIN_NTQ {
                if (fcan_div % ntq) == 0 {
                    div8 = 0;
                    prescaler = fcan_div / ntq;
                    if (prescaler > 0) && (prescaler <= NODE_MAX_PRESCALER) {
                        if (prescaler & 0x7 != 0) {
                            ntq -= 1;
                            continue;
                        } else {
                            div8 = 1;
                        }
                    }
                    tseg1 = (ntq - 1) * cfg.sample_point / 10000;
                    tseg2 = ntq - tseg1 - 1;

                    if (NODE_MIN_TSEG1 <= tseg1
                        && tseg1 <= NODE_MAX_TSEG2
                        && NODE_MIN_TSEG2 <= tseg2
                        && tseg2 < NODE_MAX_TSEG2
                        && tseg2 >= cfg.sjw)
                    {
                        break;
                    }
                }
                ntq -= 1;
            }

            if (ntq >= NODE_MIN_NTQ) {
                // TODO Add assertions maybe

                self.node_enable_configuration_change();

                self.regs.clc()
            }
        }
        status
    }

    fn node_enable_configuration_change(&self) {}

    fn node_disable_configuration_change(&self) {}
}
