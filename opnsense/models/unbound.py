from typing import Optional

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.dnssec import Dnssec
from opnsense.models.dnssecstripped import Dnssecstripped
from opnsense.models.enable import Enable

__NAMESPACE__ = "https://opnsense.org/config"


class Unbound(BaseModel):
    class Meta:
        name = "unbound"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    enable: Enable = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    dnssec: Dnssec | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    dnssecstripped: Dnssecstripped | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
