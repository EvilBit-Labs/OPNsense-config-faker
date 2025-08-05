from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.ctrl_agent import CtrlAgent
from opnsense.models.dhcp4 import Dhcp4

__NAMESPACE__ = "https://opnsense.org/config"


class Kea(BaseModel):
    class Meta:
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    ctrl_agent: CtrlAgent = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    dhcp4: Dhcp4 = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
