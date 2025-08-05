from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.pipes import Pipes
from opnsense.models.queues import Queues
from opnsense.models.rules import Rules

__NAMESPACE__ = "https://opnsense.org/config"


class TrafficShaper(BaseModel):
    class Meta:
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    pipes: Pipes = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    queues: Queues = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    rules: Rules = field(
        metadata={
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
