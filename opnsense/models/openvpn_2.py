from typing import Optional

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.descr import Descr
from opnsense.models.enable import Enable
from opnsense.models.if_mod import If
from opnsense.models.internal_dynamic import InternalDynamic
from opnsense.models.networks import Networks
from opnsense.models.type_mod import Type
from opnsense.models.virtual import Virtual

__NAMESPACE__ = "https://opnsense.org/config"


class Openvpn2(BaseModel):
    class Meta:
        name = "openvpn"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    internal_dynamic: InternalDynamic | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    enable: Enable | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    if_value: If | None = field(
        default=None,
        metadata={
            "name": "if",
            "type": "Element",
            "namespace": "",
        },
    )
    descr: Descr | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    type_value: Type | None = field(
        default=None,
        metadata={
            "name": "type",
            "type": "Element",
            "namespace": "",
        },
    )
    virtual: Virtual | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    networks: Networks | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
