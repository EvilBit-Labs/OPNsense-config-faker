from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.app import App
from opnsense.models.asn import Asn
from opnsense.models.cfg import Cfg
from opnsense.models.chd import Chd
from opnsense.models.dmn import Dmn
from opnsense.models.enc import Enc
from opnsense.models.esp import Esp
from opnsense.models.ike import Ike
from opnsense.models.ike_name import IkeName
from opnsense.models.imc import Imc
from opnsense.models.imv import Imv
from opnsense.models.job import Job
from opnsense.models.knl import Knl
from opnsense.models.lib import Lib
from opnsense.models.log_level import LogLevel
from opnsense.models.mgr import Mgr
from opnsense.models.net import Net
from opnsense.models.pts import Pts
from opnsense.models.tls import Tls
from opnsense.models.tnc import Tnc

__NAMESPACE__ = "https://opnsense.org/config"


class Daemon(BaseModel):
    class Meta:
        name = "daemon"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    ike_name: IkeName = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    log_level: LogLevel = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    app: App = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    asn: Asn = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    cfg: Cfg = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    chd: Chd = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    dmn: Dmn = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    enc: Enc = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    esp: Esp = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    ike: Ike = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    imc: Imc = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    imv: Imv = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    job: Job = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    knl: Knl = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    lib: Lib = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    mgr: Mgr = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    net: Net = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    pts: Pts = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    tls: Tls = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    tnc: Tnc = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
