from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.descr import Descr
from opnsense.models.enable import Enable
from opnsense.models.if_mod import If
from opnsense.models.internal_dynamic import InternalDynamic
from opnsense.models.ipaddr import Ipaddr
from opnsense.models.ipaddrv6 import Ipaddrv6
from opnsense.models.subnet import Subnet
from opnsense.models.subnetv6 import Subnetv6
from opnsense.models.type_mod import Type
from opnsense.models.virtual import Virtual

__NAMESPACE__ = "https://opnsense.org/config"


class Lo0(BaseModel):
    class Meta:
        name = "lo0"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    internal_dynamic: InternalDynamic = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    descr: Descr = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    enable: Enable = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    if_value: If = field(
        metadata={
            "name": "if",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    ipaddr: Ipaddr = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    ipaddrv6: Ipaddrv6 = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    subnet: Subnet = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    subnetv6: Subnetv6 = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    type_value: Type = field(
        metadata={
            "name": "type",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    virtual: Virtual = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
