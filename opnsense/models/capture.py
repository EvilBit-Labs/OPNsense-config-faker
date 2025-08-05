from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.egress_only import EgressOnly
from opnsense.models.interfaces_2 import Interfaces2
from opnsense.models.targets import Targets
from opnsense.models.version import Version

__NAMESPACE__ = "https://opnsense.org/config"


class Capture(BaseModel):
    class Meta:
        name = "capture"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    interfaces: Interfaces2 = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    egress_only: EgressOnly = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    version: Version = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    targets: Targets = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
