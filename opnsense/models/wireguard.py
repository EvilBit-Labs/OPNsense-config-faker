from collections.abc import Iterable
from typing import Union

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.client import Client
from opnsense.models.descr import Descr
from opnsense.models.enable import Enable
from opnsense.models.general import General
from opnsense.models.if_mod import If
from opnsense.models.internal_dynamic import InternalDynamic
from opnsense.models.servers import Server
from opnsense.models.type_mod import Type
from opnsense.models.virtual import Virtual

__NAMESPACE__ = "https://opnsense.org/config"


class Wireguard(BaseModel):
    class Meta:
        name = "wireguard"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    choice: Iterable[
        (
            InternalDynamic |
            Enable |
            If |
            Descr |
            Type |
            Virtual |
            Client |
            General |
            Server
        )
    ] = field(
        default_factory=list,
        metadata={
            "type": "Elements",
            "choices": (
                {
                    "name": "internal_dynamic",
                    "type": InternalDynamic,
                    "namespace": "",
                },
                {
                    "name": "enable",
                    "type": Enable,
                    "namespace": "",
                },
                {
                    "name": "if",
                    "type": If,
                    "namespace": "",
                },
                {
                    "name": "descr",
                    "type": Descr,
                    "namespace": "",
                },
                {
                    "name": "type",
                    "type": Type,
                    "namespace": "",
                },
                {
                    "name": "virtual",
                    "type": Virtual,
                    "namespace": "",
                },
                {
                    "name": "client",
                    "type": Client,
                    "namespace": "",
                },
                {
                    "name": "general",
                    "type": General,
                    "namespace": "",
                },
                {
                    "name": "server",
                    "type": Server,
                    "namespace": "",
                },
            ),
        },
    )
