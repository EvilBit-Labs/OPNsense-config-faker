from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.ignore_acquire_ts import IgnoreAcquireTs
from opnsense.models.ikesa_table_segments import IkesaTableSegments
from opnsense.models.ikesa_table_size import IkesaTableSize
from opnsense.models.init_limit_half_open import InitLimitHalfOpen
from opnsense.models.make_before_break import MakeBeforeBreak
from opnsense.models.max_ikev1_exchanges import MaxIkev1Exchanges
from opnsense.models.retransmit_base import RetransmitBase
from opnsense.models.retransmit_jitter import RetransmitJitter
from opnsense.models.retransmit_limit import RetransmitLimit
from opnsense.models.retransmit_timeout import RetransmitTimeout
from opnsense.models.retransmit_tries import RetransmitTries
from opnsense.models.syslog_2 import Syslog2
from opnsense.models.threads import Threads

__NAMESPACE__ = "https://opnsense.org/config"


class Charon(BaseModel):
    class Meta:
        name = "charon"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    max_ikev1_exchanges: MaxIkev1Exchanges = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    threads: Threads = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    ikesa_table_size: IkesaTableSize = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    ikesa_table_segments: IkesaTableSegments = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    init_limit_half_open: InitLimitHalfOpen = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    ignore_acquire_ts: IgnoreAcquireTs = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    make_before_break: MakeBeforeBreak = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    retransmit_tries: RetransmitTries = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    retransmit_timeout: RetransmitTimeout = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    retransmit_base: RetransmitBase = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    retransmit_jitter: RetransmitJitter = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    retransmit_limit: RetransmitLimit = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    syslog: Syslog2 = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
