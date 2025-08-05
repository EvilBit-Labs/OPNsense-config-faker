from collections.abc import Iterable
from typing import Optional, Union

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.blockbogons import Blockbogons
from opnsense.models.blockpriv import Blockpriv
from opnsense.models.descr import Descr
from opnsense.models.dhcp6_ia_pd_len import Dhcp6IaPdLen
from opnsense.models.dhcphostname import Dhcphostname
from opnsense.models.enable import Enable
from opnsense.models.gateway import Gateway
from opnsense.models.gatewayv6 import Gatewayv6
from opnsense.models.if_mod import If
from opnsense.models.ipaddr import Ipaddr
from opnsense.models.ipaddrv6 import Ipaddrv6
from opnsense.models.media import Media
from opnsense.models.mediaopt import Mediaopt
from opnsense.models.mtu import Mtu
from opnsense.models.spoofmac import Spoofmac
from opnsense.models.subnet import Subnet
from opnsense.models.subnetv6 import Subnetv6

__NAMESPACE__ = "https://opnsense.org/config"


class Wan(BaseModel):
    class Meta:
        name = "wan"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    descr_or_enable_or_if: Iterable[Descr | Enable | If] = field(
        default_factory=list,
        metadata={
            "type": "Elements",
            "choices": (
                {
                    "name": "descr",
                    "type": Descr,
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
            ),
        },
    )
    mtu_or_spoofmac: Mtu | Spoofmac | None = field(
        default=None,
        metadata={
            "type": "Elements",
            "choices": (
                {
                    "name": "mtu",
                    "type": Mtu,
                    "namespace": "",
                },
                {
                    "name": "spoofmac",
                    "type": Spoofmac,
                    "namespace": "",
                },
            ),
        },
    )
    choice: Iterable[
        Gateway | Ipaddr | Ipaddrv6 | Subnet | Blockbogons | Blockpriv
    ] = field(
        default_factory=list,
        metadata={
            "type": "Elements",
            "choices": (
                {
                    "name": "gateway",
                    "type": Gateway,
                    "namespace": "",
                },
                {
                    "name": "ipaddr",
                    "type": Ipaddr,
                    "namespace": "",
                },
                {
                    "name": "ipaddrv6",
                    "type": Ipaddrv6,
                    "namespace": "",
                },
                {
                    "name": "subnet",
                    "type": Subnet,
                    "namespace": "",
                },
                {
                    "name": "blockbogons",
                    "type": Blockbogons,
                    "namespace": "",
                },
                {
                    "name": "blockpriv",
                    "type": Blockpriv,
                    "namespace": "",
                },
            ),
        },
    )
    dhcphostname: Dhcphostname | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    media: Media | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    mediaopt: Mediaopt | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    dhcp6_ia_pd_len: Dhcp6IaPdLen | None = field(
        default=None,
        metadata={
            "name": "dhcp6-ia-pd-len",
            "type": "Element",
            "namespace": "",
        },
    )
    subnetv6: Subnetv6 | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    gatewayv6: Gatewayv6 | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
