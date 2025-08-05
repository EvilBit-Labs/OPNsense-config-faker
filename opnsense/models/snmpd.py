from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.rocommunity import Rocommunity
from opnsense.models.syscontact import Syscontact
from opnsense.models.syslocation import Syslocation

__NAMESPACE__ = "https://opnsense.org/config"


class Snmpd(BaseModel):
    class Meta:
        name = "snmpd"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    syslocation: Syslocation = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    syscontact: Syscontact = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    rocommunity: Rocommunity = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
