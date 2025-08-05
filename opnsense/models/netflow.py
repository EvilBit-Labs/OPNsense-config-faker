from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.active_timeout import ActiveTimeout
from opnsense.models.capture import Capture
from opnsense.models.enable import Enable
from opnsense.models.inactive_timeout import InactiveTimeout

__NAMESPACE__ = "https://opnsense.org/config"


class Netflow(BaseModel):
    class Meta:
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    capture: Capture = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    enable: Enable = field(
        metadata={
            "wrapper": "collect",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    active_timeout: ActiveTimeout = field(
        metadata={
            "name": "activeTimeout",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    inactive_timeout: InactiveTimeout = field(
        metadata={
            "name": "inactiveTimeout",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    version: str = field(
        metadata={
            "type": "Attribute",
            "required": True,
        }
    )
