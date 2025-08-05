from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.crt import Crt
from opnsense.models.descr import Descr
from opnsense.models.prv import Prv
from opnsense.models.refid import Refid

__NAMESPACE__ = "https://opnsense.org/config"


class Cert(BaseModel):
    class Meta:
        name = "cert"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    refid: Refid = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    descr: Descr = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    crt: Crt = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    prv: Prv = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
