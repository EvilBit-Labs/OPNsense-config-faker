from collections.abc import Iterable

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.mode import Mode
from opnsense.models.rule import Rule

__NAMESPACE__ = "https://opnsense.org/config"


class Outbound(BaseModel):
    class Meta:
        name = "outbound"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    mode: Mode = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    rule: Iterable[Rule] = field(
        default_factory=list,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
