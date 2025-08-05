from typing import Optional

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.flavour import Flavour
from opnsense.models.mirror import Mirror
from opnsense.models.plugins import Plugins
from opnsense.models.reboot import Reboot
from opnsense.models.subscription import Subscription
from opnsense.models.type_mod import Type

__NAMESPACE__ = "https://opnsense.org/config"


class Firmware(BaseModel):
    class Meta:
        name = "firmware"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    mirror: Mirror = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    flavour: Flavour = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    plugins: Plugins = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    type_value: Type | None = field(
        default=None,
        metadata={
            "name": "type",
            "type": "Element",
            "namespace": "",
        },
    )
    subscription: Subscription | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    reboot: Reboot | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    version: str = field(
        metadata={
            "type": "Attribute",
            "required": True,
        }
    )
