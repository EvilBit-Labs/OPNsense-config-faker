from typing import Optional, Union

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.any import AnyType
from opnsense.models.network import Network

__NAMESPACE__ = "https://opnsense.org/config"


class Source(BaseModel):
    class Meta:
        name = "source"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    any_or_network: AnyType | Network | None = field(
        default=None,
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
            ),
        },
    )
