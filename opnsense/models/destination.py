from collections.abc import Iterable
from typing import Union

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.any import AnyType
from opnsense.models.network import Network
from opnsense.models.port import Port

__NAMESPACE__ = "https://opnsense.org/config"


class Destination(BaseModel):
    class Meta:
        name = "destination"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    any_or_network_or_port: Iterable[AnyType | Network | Port] = field(
        default_factory=list,
        metadata={
            "type": "Elements",
            "choices": (
                {
                    "name": "any",
                    "type": AnyType,
                    "namespace": "",
                },
                {
                    "name": "network",
                    "type": Network,
                    "namespace": "",
                },
                {
                    "name": "port",
                    "type": Port,
                    "namespace": "",
                },
            ),
            "max_occurs": 2,
        },
    )
